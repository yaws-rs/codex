//! Types

#[cfg(feature = "fixed")]
mod fixed {
    /// Fixed size Encoder
    #[derive(Debug)]
    #[cfg(feature = "encode")]
    pub struct FixedEncoder<'i, const S: usize> {
        pub(crate) cur_block: [u8; S],
        pub(crate) cur_raw: Option<&'i str>,
    }

    /// Fixed size Decoder
    #[derive(Debug)]
    #[cfg(feature = "decode")]
    pub struct FixedDecoder<'o, const S: usize> {
        #[allow(dead_code)]
        pub(crate) cur_block: [u8; S],
        #[allow(dead_code)]
        pub(crate) cur_raw: Option<&'o str>,
    }
}

#[cfg(feature = "fixed")]
pub use fixed::*;

#[cfg(all(feature = "vec", any(feature = "alloc", feature = "std")))]
mod vec {

    /// Vec Encoder
    #[derive(Debug)]
    #[cfg(feature = "encode")]
    pub struct VecEncoder;

    /// Vec Decoder
    #[derive(Debug)]
    #[cfg(feature = "decode")]
    pub struct VecDecoder;
}

#[cfg(all(feature = "vec", any(feature = "alloc", feature = "std")))]
pub use vec::*;
