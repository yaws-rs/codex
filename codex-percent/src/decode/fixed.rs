//! Decode impl - Fixed size array output

use super::*;
use crate::{DecoderError, FixedDecoder as Decoder};

impl<'i, const S: usize> Decoder<'i, S> {
    /// Init decoder
    pub fn init() -> Self {
        let cur: [u8; S] = [0; S];
        Self {
            cur_block: cur,
            cur_raw: None,
        }
    }
    /// Current block output
    pub fn cur_block(&self) -> &[u8; S] {
        &self.cur_block
    }
    /// Current raw input
    pub fn cur_raw(&self) -> Option<&str> {
        self.cur_raw
    }
    /// Decode and return resulting decoded size
    /// *Panic*
    /// This will panic if the output would go out of bounds
    pub fn decode(&mut self, raw: &'i str) -> Result<usize, DecoderError> {
        let mut cur_i = 0;
        self.cur_raw = Some(raw);

        let mut lexer = DecodeToken::lexer(raw);
        while let Some(token) = lexer.next() {
            match token {
                Ok(DecodeToken::MaybePercent2Hex(pairs)) => {
                    let mut chunks = pairs.as_bytes().chunks(3);
                    while let Some(block) = chunks.next() {
                        let pair: [u8; 2] = [block[1], block[2]];
                        let byte_out = hex2byte(pair)?;
                        self.cur_block[cur_i] = byte_out;
                        cur_i = cur_i + 1;
//                        panic!("Ok got chunk {:?}", pair);
                    }
                }
                /*
                Ok(DecodeToken::Unreserved(alphanum)) => {
                    let needed = alphanum.as_bytes().len();
                    let fill_to = cur_i + needed;
                    self.cur_block[cur_i..fill_to].copy_from_slice(&alphanum.as_bytes()[0..needed]);
                    cur_i += needed;
                }
                Ok(DecodeToken::NotUnreserved(notalphanum)) => {
                    let mut bytes = notalphanum.bytes();
                    while let Some(a) = bytes.next() {
                        let (higher, lower) = byte2hex(a, &HEX_CHARS_UPPER);
                        let needed = 3;
                        self.cur_block[cur_i] = 37;
                        self.cur_block[cur_i + 1] = higher;
                        self.cur_block[cur_i + 2] = lower;
                        cur_i = cur_i + needed;
                    }
            }
                */
                _ => {
                    return Err(DecoderError::BorkedExperimental(lexer.span().start));
                }
            }
        }
        Ok(cur_i)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn f_10_poop() {
        let poop_str = "%F0%9F%92%A9";
        let expected: [u8; 4] = [240, 159, 146, 169];
        let mut d = Decoder::<4>::init();
        let c = d.decode(poop_str).unwrap();
        let utf8 = core::str::from_utf8(d.cur_block());

        assert_eq!(c, 4);
        assert_eq!(d.cur_block(), &expected);
        assert_eq!(utf8, Ok("💩"));
    }
/*
    #[test]
    fn f_10_direct_copy() {
        let s = "1234567890";

        let mut e = Decoder::<10>::init();
        let f = e.encode(s).unwrap();

        assert_eq!(f, 10);
        assert_eq!(e.cur_block, [49, 50, 51, 52, 53, 54, 55, 56, 57, 48]);
    }
    #[test]
    fn f_10_poop() {
        let s = "💩";

        let mut e = Decoder::<12>::init();
        let res = e.encode(s).unwrap();

        assert_eq!(res, 12);
        let t = core::str::from_utf8(e.cur_block.as_slice());

        assert_eq!(t, Ok("%F0%9F%92%A9"));
    }
    #[test]
    fn f_10_spaces() {
        let s = "          ";

        let mut e = Decoder::<30>::init();
        let f = e.encode(s).unwrap();

        assert_eq!(f, 30);

        let t = core::str::from_utf8(e.cur_block.as_slice());
        assert_eq!(t, Ok("%20%20%20%20%20%20%20%20%20%20"));

        assert_eq!(
            e.cur_block,
            [
                37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48,
                37, 50, 48, 37, 50, 48, 37, 50, 48
            ]
        );
    }
 */
}

