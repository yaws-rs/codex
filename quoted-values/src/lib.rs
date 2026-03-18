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

//#[cfg(all(feature = "alloc", not(feature = "std")))]
//extern crate alloc;

#[cfg(all(not(feature = "decode"), not(feature = "encode")))]
compile_error!("Pick features decode and / or encode as neither is default on.");

//---------------------------------------------------------
// Re-exports on external types we may use
//---------------------------------------------------------

//---------------------------------------------------------
// Error types
//---------------------------------------------------------

mod error;
#[doc(inline)]
pub use error::*;

//--------------------------------------------------------
// Encoding implementations with type conversions
//--------------------------------------------------------

//#[cfg(feature = "encode")]
//pub(crate) mod encode;

//--------------------------------------------------------
// Decoding implementations with type conversions
//--------------------------------------------------------

#[cfg(feature = "decode")]
mod decode;
#[doc(inline)]
#[cfg(feature = "decode")]
pub use decode::*;
