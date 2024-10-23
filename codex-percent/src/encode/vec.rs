//! Encode impl - Vec allocated output

use super::*;
use crate::{EncoderError, VecEncoder as Encoder};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::*;

impl Encoder {
    /// Encode and append result into the provided &mut Vec
    /// Result Ok() Provides the written bytes
    pub fn encode(raw: &str, vin: &mut Vec<u8>) -> Result<usize, EncoderError> {
        let mut cur_i = 0;

        let mut lexer = EncodeToken::lexer(raw);
        while let Some(token) = lexer.next() {
            match token {
                Ok(EncodeToken::Unreserved(alphanum)) => {
                    let needed = alphanum.as_bytes().len();
                    vin.extend_from_slice(&alphanum.as_bytes()[0..needed]);
                    cur_i += needed;
                }
                Ok(EncodeToken::NotUnreserved(notalphanum)) => {
                    let mut bytes = notalphanum.bytes();
                    while let Some(a) = bytes.next() {
                        let (higher, lower) = byte2hex(a, &HEX_CHARS_UPPER);
                        let needed = 3;
                        vin.push(37);
                        vin.push(higher);
                        vin.push(lower);
                        cur_i = cur_i + needed;
                    }
                }
                _ => {
                    return Err(EncoderError::BorkedExperimental(lexer.span().start));
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
    fn f_10_direct_copy_no_cap() {
        let s = "1234567890";
        let mut v: Vec<u8> = Vec::new();

        let f = Encoder::encode(s, &mut v).unwrap();

        assert_eq!(f, 10);
        assert_eq!(v, [49, 50, 51, 52, 53, 54, 55, 56, 57, 48]);
    }
    #[test]
    fn f_10_direct_copy_10_cap() {
        let s = "1234567890";

        let mut v: Vec<u8> = Vec::with_capacity(10);
        let f = Encoder::encode(s, &mut v).unwrap();

        assert_eq!(f, 10);
        assert_eq!(v, [49, 50, 51, 52, 53, 54, 55, 56, 57, 48]);
    }
    #[test]
    fn f_10_poop() {
        let s = "💩";

        let mut v: Vec<u8> = Vec::new();
        let f = Encoder::encode(s, &mut v).unwrap();

        assert_eq!(f, 12);
        let t = core::str::from_utf8(v.as_slice());

        assert_eq!(t, Ok("%F0%9F%92%A9"));
    }
    #[test]
    fn f_10_spaces() {
        let s = "          ";

        let mut v: Vec<u8> = Vec::new();
        let f = Encoder::encode(s, &mut v).unwrap();

        assert_eq!(f, 30);

        let t = core::str::from_utf8(v.as_slice());
        assert_eq!(t, Ok("%20%20%20%20%20%20%20%20%20%20"));

        assert_eq!(
            v,
            [
                37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48,
                37, 50, 48, 37, 50, 48, 37, 50, 48
            ]
        );
    }
}
