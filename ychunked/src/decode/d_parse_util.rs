//! Parse from hex to usize in lexer callback

use super::ChunkSizeToken;
use logos::Lexer;

#[inline]
pub(crate) fn parse_usize_radix16(lex: &mut Lexer<'_, ChunkSizeToken>) -> Option<usize> {
    let slice = match lex.slice().strip_suffix(&[13, 10]) {
        Some(s) => s,
        _ => return None,
    };

    let s = match core::str::from_utf8(slice) {
        Ok(s) => s,
        _ => return None,
    };

    match usize::from_str_radix(s, 16) {
        Ok(0) => None,
        Ok(u) => Some(u),
        _ => None,
    }
}
