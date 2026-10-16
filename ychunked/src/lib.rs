#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

//---------------------------------------------------------
// Re-exports on external types we may use
//---------------------------------------------------------

//---------------------------------------------------------
// Error types
//---------------------------------------------------------

mod error;
#[doc(inline)]
pub use error::*;

//---------------------------------------------------------
// pub types
//---------------------------------------------------------
mod decode;
#[doc(inline)]
pub use decode::*;
