//! yChunked Error

/// Encoder Errors
#[derive(Debug, PartialEq)]
pub enum EncoderError {}

/// Decoder Errors
#[derive(Debug, PartialEq)]
pub enum DecoderError {
    /// Unexpected input at
    UnexpectedInput(usize),
}
