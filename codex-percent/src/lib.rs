#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![allow(clippy::single_match, rustdoc::bare_urls)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

//---------------------------------------------------------
// Re-exports on external types we may use
//---------------------------------------------------------

//---------------------------------------------------------
// Error types
//---------------------------------------------------------

mod error;
pub use error::*;

//---------------------------------------------------------
// pub types
//---------------------------------------------------------

mod types;
#[cfg(any(feature = "encode", feature = "decode"))]
pub use types::*;

//--------------------------------------------------------
// Encoding implementations with type conversions
//--------------------------------------------------------

#[cfg(feature = "encode")]
pub(crate) mod encode;

//--------------------------------------------------------
// Builder implementations with type conversions
//--------------------------------------------------------

#[cfg(feature = "decode")]
pub(crate) mod decode;
