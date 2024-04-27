mod error;

pub use error::PsDeflateError;

use std::cell::Cell;

fn alloc_compressor() -> libdeflater::Compressor {
    let level = libdeflater::CompressionLvl::best();
    let compressor = libdeflater::Compressor::new(level);

    return compressor;
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

    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, PsDeflateError> {
        let out_size = data.len() + 5;
        let mut compressor = get_compressor(&self.compressor);
        let mut out_data = Vec::with_capacity(out_size);

        unsafe { out_data.set_len(out_size) };

        let result = compressor.deflate_compress(data, out_data.as_mut_slice());

        put_compressor(&self.compressor, compressor);

        let size = result?;

        if size < out_size {
            unsafe { out_data.set_len(size) };
        }

        Ok(out_data)
    }

    pub fn decompress(&self, data: &[u8], out_size: usize) -> Result<Vec<u8>, PsDeflateError> {
        let mut out_data = Vec::with_capacity(out_size);

        unsafe { out_data.set_len(out_size) };

        let mut decompressor = get_decompressor(&self.decompressor);

        let result = decompressor.deflate_decompress(data, out_data.as_mut_slice());

        put_decompressor(&self.decompressor, decompressor);

        let size = result?;

        if size < out_size {
            unsafe { out_data.set_len(size) };
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
