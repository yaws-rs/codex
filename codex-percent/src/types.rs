//! Types

/// Fixed size Encoder
#[derive(Debug)]
pub struct Encoder<'i, const S: usize> {
    pub(crate) cur_block: [u8; S],
    pub(crate) cur_size: usize,
    pub(crate) raw: Option<&'i str>,
}

/// Fixed size Decoder
#[derive(Debug)]
pub struct Decoder<'o, const S: usize> {
    pub(crate) cur_block: [u8; S],
    pub(crate) raw: &'o str,    
}

