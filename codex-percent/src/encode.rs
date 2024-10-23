//! Encode impl

#[cfg(feature = "fixed")]
mod fixed;

#[cfg(feature = "vec")]
mod vec;

use logos::Logos;

#[derive(Debug, Logos)]
enum EncodeToken<'i> {
    #[regex("[A-Za-z0-9-._~]+", |lex| lex.slice(), priority = 100)]
    Unreserved(&'i str),

    #[regex("[^A-Za-z0-9-._~]+", |lex| lex.slice(), priority = 50)]
    NotUnreserved(&'i str),
}

const HEX_CHARS_UPPER: &[u8; 16] = b"0123456789ABCDEF";

#[inline]
#[must_use]
const fn byte2hex(byte: u8, table: &[u8; 16]) -> (u8, u8) {
    let high = table[((byte & 0xf0) >> 4) as usize];
    let low = table[(byte & 0x0f) as usize];

    (high, low)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn byte2hex_sp() {
        let b: u8 = " ".bytes().nth(0).expect("w");
        let t = byte2hex(b, HEX_CHARS_UPPER);
        assert_eq!(t, (50, 48));
    }
}
