//! Decode impls

#[cfg(feature = "fixed")]
mod fixed;

#[cfg(feature = "vec")]
mod vec;

use logos::Logos;

use crate::DecoderError;

#[derive(Debug, Logos)]
enum DecodeToken<'o> {
    #[regex("(%[0-9A-Fa-f]{2}){1,}", |lex| lex.slice(), priority = 100)]
    MaybePercent2Hex(&'o str),

    #[regex("[^%]+", |lex| lex.slice(), priority = 50)]
    EverythingElse(&'o str),
}

#[inline]
#[must_use]
const fn val(c: u8) -> Result<u8, DecoderError> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(DecoderError::InternalConversion),
    }
}

#[inline]
#[must_use]
fn hex2byte(pair: [u8; 2]) -> Result<u8, DecoderError> {
    Ok(val(pair[0])? << 4 | val(pair[1])?)
}

#[cfg(test)]
mod test {

    use super::*;
    
    #[test]
    fn hex2bytes_sp() {
	    let h: [u8; 2] = [50, 48];
	    let b = hex2byte(h);
	    assert_eq!(b, Ok(32));
    }    
}
