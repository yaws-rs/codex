//! Quoted-values Error

/// Encoder Errors
#[derive(Debug, PartialEq)]
#[cfg(feature = "encode")]
pub enum EncodeError {}

/// Decoder Errors
#[cfg(feature = "decode")]
#[derive(Debug, PartialEq)]
pub enum DecodeError {
    /// Unexpected character at position
    Unexpected(usize),
    /// Encountered bug
    Bug(&'static str),
}
