//! Encode impl

use crate::{Encoder, EncoderError};

use logos::{Lexer, Logos};

#[derive(Debug, Logos)]
enum EncodeToken<'i> {
    #[regex("[A-Za-z0-9-._~]+", |lex| lex.slice(), priority = 100)]
    Unreserved(&'i str),

    #[regex("[^A-Za-z0-9-._~]+", |lex| lex.slice(), priority = 50)]
    NotUnreserved(&'i str),
}

const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";
const HEX_CHARS_UPPER: &[u8; 16] = b"0123456789ABCDEF";

#[inline]
#[must_use]
const fn byte2hex(byte: u8, table: &[u8; 16]) -> (u8, u8) {
    let high = table[((byte & 0xf0) >> 4) as usize];
    let low = table[(byte & 0x0f) as usize];

    (high, low)
}

impl<'i, const S: usize> Encoder<'i, S> {
    fn init() -> Self {
        let cur: [u8; S] = [0; S];
        Self { cur_block: cur, raw: None, cur_size: 0 }
    }
    fn encode(&mut self, raw: &'i str) -> Result<usize, EncoderError> {
        let mut cur_i = 0;
        
        let mut lexer = EncodeToken::lexer(raw);
        while let Some(token) = lexer.next() {
            match token {
                Ok(EncodeToken::Unreserved(alphanum)) => {
                    let needed = alphanum.as_bytes().len();
                    let fill_to = cur_i + needed;
                    self.cur_block[cur_i..fill_to].copy_from_slice(&alphanum.as_bytes()[0..needed]);
                    cur_i += needed;
                }
                Ok(EncodeToken::NotUnreserved(notalphanum)) => {
                    let mut bytes = notalphanum.bytes();
                    while let Some(a) = bytes.next() {
                        let (higher, lower) = byte2hex(a, &HEX_CHARS_UPPER);
                        let needed = 3;
                        self.cur_block[cur_i] = 37;
                        self.cur_block[cur_i+1] = higher;
                        self.cur_block[cur_i+2] = lower;
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
    fn f_10_direct_copy() {
        let s = "1234567890";

        let mut e = Encoder::<10>::init();
        let f = e.encode(s).unwrap();

        assert_eq!(f, 10);
        assert_eq!(e.cur_block, [49, 50, 51, 52, 53, 54, 55, 56, 57, 48]);
    }
    #[test]
    fn f_10_poop() {
        let s = "💩";

        let mut e = Encoder::<12>::init();
        let res = e.encode(s).unwrap();

        assert_eq!(res, 12);
        let t = core::str::from_utf8(e.cur_block.as_slice());

        assert_eq!(t, Ok("%F0%9F%92%A9"));
    }
    #[test]
    fn f_10_spaces() {
        let s = "          ";

        let mut e = Encoder::<30>::init();
        let f = e.encode(s).unwrap();

        assert_eq!(f, 30);

        let t = core::str::from_utf8(e.cur_block.as_slice());
        assert_eq!(t, Ok("%20%20%20%20%20%20%20%20%20%20"));

        assert_eq!(e.cur_block, [37, 50, 48, 37, 50, 48, 37, 50, 48, 37,
                                 50, 48, 37, 50, 48, 37, 50, 48, 37, 50,
                                 48, 37, 50, 48, 37, 50, 48, 37, 50, 48]);
    }
    
}
