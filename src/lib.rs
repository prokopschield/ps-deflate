mod error;

pub use error::PsDeflateError;

use std::cell::RefCell;

use libdeflater::{CompressionLvl, Compressor, Decompressor};
use ps_buffer::Buffer;

thread_local! {
    pub static COMPRESSOR: RefCell<Compressor> = RefCell::from(Compressor::new(CompressionLvl::best()));
    pub static DECOMPRESSOR: RefCell<Decompressor> = RefCell::from(Decompressor::new());
}

pub fn compress_into(data: &[u8], out_data: &mut [u8]) -> Result<usize, PsDeflateError> {
    COMPRESSOR.with(|c| Ok(c.borrow_mut().deflate_compress(data, out_data)?))
}

pub fn compress(data: &[u8]) -> Result<Buffer, PsDeflateError> {
    let out_size = data.len() + 5;
    let mut out_data = Buffer::alloc(out_size);

    let size = compress_into(data, &mut out_data)?;

    if size < out_size {
        out_data.resize(size);
    }

    Ok(out_data)
}

pub fn decompress_into(data: &[u8], out_data: &mut [u8]) -> Result<usize, PsDeflateError> {
    DECOMPRESSOR.with(|d| Ok(d.borrow_mut().deflate_decompress(data, out_data)?))
}

pub fn decompress(data: &[u8], out_size: usize) -> Result<Buffer, PsDeflateError> {
    let mut out_data = Buffer::alloc(out_size);

    let size = decompress_into(data, &mut out_data)?;

    if size < out_size {
        out_data.resize(size);
    }

    Ok(out_data)
}
