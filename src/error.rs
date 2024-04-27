pub enum PsDeflateError {
    BadData,
    InsufficientSpace,
}

impl From<libdeflater::CompressionError> for PsDeflateError {
    fn from(error: libdeflater::CompressionError) -> Self {
        match error {
            libdeflater::CompressionError::InsufficientSpace => PsDeflateError::InsufficientSpace,
        }
    }
}

impl From<libdeflater::DecompressionError> for PsDeflateError {
    fn from(error: libdeflater::DecompressionError) -> Self {
        match error {
            libdeflater::DecompressionError::BadData => PsDeflateError::BadData,
            libdeflater::DecompressionError::InsufficientSpace => PsDeflateError::InsufficientSpace,
        }
    }
}