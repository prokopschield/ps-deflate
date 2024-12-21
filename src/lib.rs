mod error;

pub use error::PsDeflateError;
use ps_buffer::Buffer;

use std::cell::Cell;

fn alloc_compressor() -> libdeflater::Compressor {
    let level = libdeflater::CompressionLvl::best();

    libdeflater::Compressor::new(level)
}

fn get_compressor(cell: &Cell<Option<libdeflater::Compressor>>) -> libdeflater::Compressor {
    match cell.replace(None) {
        Some(compressor) => compressor,
        None => alloc_compressor(),
    }
}

fn put_compressor(
    cell: &Cell<Option<libdeflater::Compressor>>,
    compressor: libdeflater::Compressor,
) {
    cell.replace(Some(compressor));
}

fn alloc_decompressor() -> libdeflater::Decompressor {
    libdeflater::Decompressor::new()
}

fn get_decompressor(cell: &Cell<Option<libdeflater::Decompressor>>) -> libdeflater::Decompressor {
    match cell.replace(None) {
        Some(compressor) => compressor,
        None => alloc_decompressor(),
    }
}

fn put_decompressor(
    cell: &Cell<Option<libdeflater::Decompressor>>,
    decompressor: libdeflater::Decompressor,
) {
    cell.replace(Some(decompressor));
}

pub struct Compressor {
    compressor: Cell<Option<libdeflater::Compressor>>,
    decompressor: Cell<Option<libdeflater::Decompressor>>,
}

impl Compressor {
    pub fn new() -> Self {
        Self {
            compressor: Cell::from(None),
            decompressor: Cell::from(None),
        }
    }

    pub fn compress_into(&self, data: &[u8], out_data: &mut [u8]) -> Result<usize, PsDeflateError> {
        let mut compressor = get_compressor(&self.compressor);

        let result = compressor.deflate_compress(data, out_data);

        put_compressor(&self.compressor, compressor);

        Ok(result?)
    }

    pub fn compress(&self, data: &[u8]) -> Result<Buffer, PsDeflateError> {
        let out_size = data.len() + 5;
        let mut out_data = Buffer::alloc(out_size);

        let size = self.compress_into(data, &mut out_data)?;

        if size < out_size {
            out_data.resize(size);
        }

        Ok(out_data)
    }

    pub fn decompress_into(
        &self,
        data: &[u8],
        out_data: &mut [u8],
    ) -> Result<usize, PsDeflateError> {
        let mut decompressor = get_decompressor(&self.decompressor);

        let result = decompressor.deflate_decompress(data, out_data);

        put_decompressor(&self.decompressor, decompressor);

        Ok(result?)
    }

    pub fn decompress(&self, data: &[u8], out_size: usize) -> Result<Buffer, PsDeflateError> {
        let mut out_data = Buffer::alloc(out_size);

        let size = self.decompress_into(data, &mut out_data)?;

        if size < out_size {
            out_data.resize(size);
        }

        Ok(out_data)
    }
}

impl Clone for Compressor {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Default for Compressor {
    fn default() -> Self {
        Self::new()
    }
}
