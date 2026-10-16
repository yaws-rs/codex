//! Decoder

use crate::DecoderError;
use logos::{Lexer, Logos};

mod d_parse_util;
use d_parse_util::parse_usize_radix16;

/// Implement to receive decoded chunked data.
/// Note that these may be split based on input data where partial chunk may be sent instead of complete chunk.
pub trait ChunkReceiver {
    /// Receive data from chunk which may be partial or full chunk.
    fn chunk_add(&mut self, _data: &[u8]) -> bool;
}

/// Indicates the amount of decoded bytes that should be discard from the input
pub type Discard = usize;

/// Indicates the amount of bytes remaining in the current chunk
pub type Remaining = usize;

/// Returned when the current input data has been processed returning
/// the current decoding state and the amount of bytes to be discarded
/// from the given input.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DecodingStage {
    /// Waiting expected size of a chunk
    WaitingSize(Discard),
    /// Expected bytes remaining for the current chunk
    Decoding(Discard, Remaining),
    /// Waiting CRLF after expeced len
    WaitingCrLf(Discard),
    /// Marks the completion of the chunked transfer
    Complete(Discard),
}

/// Chunk Decoder
#[derive(Debug, PartialEq)]
pub struct ChunkDecoder {
    stage: DecodingStage,
}

impl Default for ChunkDecoder {
    fn default() -> Self {
        Self {
            stage: DecodingStage::WaitingSize(0),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
#[allow(missing_docs)]
#[logos(utf8 = false)]
enum ChunkSizeToken {
    #[regex(r"0\r\n\r\n", priority = 300)]
    MaybeComplete,
    #[regex(r"(0|0\r|0\r\n|0\r\n\r)$", priority = 250)]
    MaybeIncompleteZero,
    // u64::MAX -> FFFFFFFFFFFFFFFF == min 2, max 16
    #[regex(r"[A-Fa-f0-9]{1,16}\r\n", callback = parse_usize_radix16, priority = 200)]
    MaybeLength(usize),
    #[regex(
        r"([A-Fa-f0-9]{1,15}|[A-Fa-f0-9]{16}|[A-Fa-f0-9](16)\r)$",
        priority = 10
    )]
    MaybeIncomplete,
}

impl ChunkDecoder {
    /// Create new instance
    pub fn new() -> Self {
        Self::default()
    }
    /// Decode in detached from input data mode
    pub fn decode_detached<R: ChunkReceiver>(
        &mut self,
        receiver: &mut R,
        mut data: &[u8],
    ) -> Result<DecodingStage, DecoderError> {
        let mut discard = 0;
        loop {
            match self.stage {
                DecodingStage::WaitingSize(_) => {
                    if data.is_empty() {
                        return Ok(DecodingStage::Complete(0));
                    }
                    let mut lexer: Lexer<'_, ChunkSizeToken> = ChunkSizeToken::lexer(data);
                    self.stage = match lexer.next() {
                        Some(Ok(ChunkSizeToken::MaybeComplete)) => {
                            discard += lexer.span().end;
                            data = &data[lexer.span().end..];
                            DecodingStage::Complete(discard)
                        }
                        /*
                        Some(Ok(ChunkSizeToken::MaybeLength(0))) => {
                            discard += lexer.span().end;
                            data = &data[lexer.span().end..];
                            DecodingStage::Complete(discard)
                        },*/
                        Some(Ok(ChunkSizeToken::MaybeLength(l))) => {
                            discard += lexer.span().end;
                            data = &data[lexer.span().end..];
                            DecodingStage::Decoding(discard, l)
                        }
                        Some(Ok(ChunkSizeToken::MaybeIncomplete))
                        | Some(Ok(ChunkSizeToken::MaybeIncompleteZero)) => {
                            return Ok(DecodingStage::WaitingSize(discard));
                        }
                        _ => {
                            return Err(DecoderError::UnexpectedInput(discard + lexer.span().start))
                        }
                    }
                }
                DecodingStage::Decoding(_, remaining) => {
                    if data.is_empty() && remaining > 0 {
                        self.stage = DecodingStage::Decoding(discard, remaining);
                        return Ok(self.stage);
                    }

                    let provide_bytes_len = if remaining > data.len() {
                        data.len()
                    } else {
                        remaining
                    };
                    let provide_b = &data[..provide_bytes_len];

                    if !receiver.chunk_add(provide_b) {
                        self.stage = DecodingStage::Decoding(discard, remaining);
                        return Ok(self.stage);
                    }
                    discard += provide_b.len();
                    data = &data[provide_bytes_len..];

                    let remaining = remaining - provide_b.len();

                    if data.is_empty() && remaining > 0 {
                        self.stage = DecodingStage::Decoding(discard, remaining);
                        return Ok(self.stage);
                    }

                    if data.is_empty() && remaining == 0 {
                        return Ok(DecodingStage::WaitingSize(discard));
                    }

                    self.stage = match remaining {
                        0 => DecodingStage::WaitingCrLf(discard),
                        _ => DecodingStage::Decoding(discard, remaining),
                    };
                }
                DecodingStage::WaitingCrLf(_) => {
                    if data.len() < 2 {
                        return Ok(DecodingStage::WaitingCrLf(discard));
                    }
                    if &data[0..2] == b"\r\n" {
                        discard += 2;
                        data = &data[2..];
                        self.stage = DecodingStage::WaitingSize(discard);
                    } else {
                        return Err(DecoderError::UnexpectedInput(discard));
                    }
                }
                DecodingStage::Complete(_) => {
                    return Ok(DecodingStage::Complete(discard));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::Context;
    use rstest::*;

    #[derive(Clone, Debug, Default, PartialEq)]
    struct TestReceiver {
        data: Vec<Vec<u8>>,
    }

    impl ChunkReceiver for TestReceiver {
        fn chunk_add(&mut self, data: &[u8]) -> bool {
            let mut nvec = Vec::with_capacity(data.len());
            nvec.extend(data);
            self.data.push(nvec);
            true
        }
    }

    #[derive(Debug, PartialEq)]
    struct TcSnapshot {
        input: &'static str,
        res: Result<DecodingStage, DecoderError>,
        discard_to: Option<&'static str>,
        receiver: TestReceiver,
    }

    pub(crate) fn ctx_insta(ctx: Context) -> String {
        let case_id = ctx.case.unwrap().to_string();
        format!("{}-{}", case_id, ctx.name)
    }

    #[rstest]
    #[case("16\r\nYeahChunkedStuffIs Fun\r\n0\r\n\r\n")]
    fn completed(#[context] ctx: Context, #[case] input: &'static str) {
        let mut receiver = TestReceiver::default();
        let mut decoder = ChunkDecoder::new();

        let res = decoder.decode_detached(&mut receiver, input.as_bytes());

        assert_eq!(res, Ok(DecodingStage::Complete(input.len())));

        insta::assert_debug_snapshot!(
            ctx_insta(ctx),
            TcSnapshot {
                input,
                res,
                discard_to: None,
                receiver
            }
        );
    }

    #[rstest]
    #[case(
        "16\r\nYeahChunkedStuffIs Fun\r\n0\r\n",
        "16\r\nYeahChunkedStuffIs Fun\r\n"
    )]
    fn incomplete_size(
        #[context] ctx: Context,
        #[case] input: &'static str,
        #[case] discard_at: &'static str,
    ) {
        let mut receiver = TestReceiver::default();
        let mut decoder = ChunkDecoder::new();

        let res = decoder.decode_detached(&mut receiver, input.as_bytes());

        let expected_discard_len = discard_at.len();
        assert_eq!(res, Ok(DecodingStage::WaitingSize(expected_discard_len)));

        let discarded = if let Ok(DecodingStage::WaitingSize(discard)) = res {
            &input[..discard]
        } else {
            panic!("Decoded result was something else than expected: {:?}", res);
        };

        assert_eq!(discarded, discard_at);

        insta::assert_debug_snapshot!(
            ctx_insta(ctx),
            TcSnapshot {
                input,
                res,
                discard_to: Some(discard_at),
                receiver
            }
        );
    }

    #[rstest]
    #[case("1\r\n", Ok(DecodingStage::Decoding(3, 1)), "1\r\n")]
    #[case("2\r\n", Ok(DecodingStage::Decoding(3, 2)), "2\r\n")]
    #[case("2\r\na", Ok(DecodingStage::Decoding(4, 1)), "2\r\na")]
    #[case(
        "1\r\na\r\n1\r\n",
        Ok(DecodingStage::Decoding(9, 1)),
        "1\r\na\r\n1\r\n"
    )]
    fn incomplete_decoding(
        #[context] ctx: Context,
        #[case] input: &'static str,
        #[case] expected: Result<DecodingStage, DecoderError>,
        #[case] discard_at: &'static str,
    ) {
        let mut receiver = TestReceiver::default();
        let mut decoder = ChunkDecoder::new();

        let res = decoder.decode_detached(&mut receiver, input.as_bytes());

        assert_eq!(res, expected);

        let discarded = if let Ok(DecodingStage::Decoding(discard, _remaining)) = res {
            &input[..discard]
        } else {
            panic!("Decoded result was something else than expected: {:?}", res);
        };

        assert_eq!(discarded, discard_at);

        insta::assert_debug_snapshot!(
            ctx_insta(ctx),
            TcSnapshot {
                input,
                res,
                discard_to: Some(discard_at),
                receiver
            }
        );
    }
}
