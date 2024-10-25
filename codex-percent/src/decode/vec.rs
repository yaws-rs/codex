//! Decode impl - Vec output

use super::*;
use crate::{DecoderError, VecDecoder as Decoder};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::*;

impl Decoder {
    /// Decode and append result into the provided &mut Vec
    /// Result Ok() Provides the written bytes
    pub fn decode(raw: &str, vin: &mut Vec<u8>) -> Result<usize, DecoderError> {
        let mut cur_i = 0;

        let mut lexer = DecodeToken::lexer(raw);
        while let Some(token) = lexer.next() {
            match token {
                Ok(DecodeToken::MaybePercent2Hex(pairs)) => {
                    let mut chunks = pairs.as_bytes().chunks(3);
                    while let Some(block) = chunks.next() {
                        let pair: [u8; 2] = [block[1], block[2]];
                        let byte_out = hex2byte(pair)?;
                        vin.push(byte_out);
                        cur_i = cur_i + 1;
                    }
                }
                Ok(DecodeToken::EverythingElse(alphanum)) => {
                    let needed = alphanum.as_bytes().len();
                    vin.extend_from_slice(&alphanum.as_bytes()[0..needed]);
                    cur_i += needed;
                }
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
    fn f_10_direct_copy() {
        let s = "1234567890";
        let expected: Vec<u8> = vec![49, 50, 51, 52, 53, 54, 55, 56, 57, 48];

        let mut d: Vec<u8> = Vec::new();
        let c = Decoder::decode(s, &mut d).unwrap();
        let utf8 = core::str::from_utf8(&d).unwrap();

        assert_eq!(c, 10);
        assert_eq!(d, expected);
        assert_eq!(utf8, s);
    }

    #[test]
    fn f_10_poop() {
        let poop_str = "%F0%9F%92%A9";
        let expected: Vec<u8> = vec![240, 159, 146, 169];
        let expected_utf8 = "💩";

        let mut d: Vec<u8> = Vec::new();
        let c = Decoder::decode(poop_str, &mut d).unwrap();
        let utf8 = core::str::from_utf8(&d).unwrap();

        assert_eq!(c, 4);
        assert_eq!(d, expected);
        assert_eq!(expected_utf8, utf8);
    }

    #[test]
    fn f_10_spaces() {
        let poop_str = "%20%20%20%20%20%20%20%20%20%20";
        let expected: Vec<u8> = vec![32, 32, 32, 32, 32, 32, 32, 32, 32, 32];
        let expected_utf8 = "          ";

        let mut d: Vec<u8> = Vec::new();
        let c = Decoder::decode(poop_str, &mut d).unwrap();
        let utf8 = core::str::from_utf8(&d).unwrap();

        assert_eq!(c, 10);
        assert_eq!(d, expected);
        assert_eq!(expected_utf8, utf8);
    }

    #[test]
    fn f_10_mixmash() {
        let poop_str = "%20 %20 %20";
        let expected: Vec<u8> = vec![32, 32, 32, 32, 32];
        let expected_utf8 = "     ";

        let mut d: Vec<u8> = Vec::new();
        let c = Decoder::decode(poop_str, &mut d).unwrap();
        let utf8 = core::str::from_utf8(&d).unwrap();

        assert_eq!(c, 5);
        assert_eq!(d, expected);
        assert_eq!(expected_utf8, utf8);
    }
}
