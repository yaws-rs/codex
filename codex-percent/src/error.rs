//! Codex-percent Error

/// Encoder Errors
#[derive(Debug, PartialEq)]
pub enum EncoderError {
    /// Encoder borked for some reason at the given position
    BorkedExperimental(usize),
}

/// Decoder Errors
#[derive(Debug, PartialEq)]
pub enum DecoderError {
    /// Decoder borked for some reason at the given position
    BorkedExperimental(usize),
    /// BUG Internal hex2dec conversion failed
    InternalConversion,
}
