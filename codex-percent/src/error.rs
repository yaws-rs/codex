//! Codex-percent Error

#[derive(Debug, PartialEq)]
pub enum EncoderError {
    /// Encoder borked for some reason at teh given position
    BorkedExperimental(usize),
}
