use ps_buffer::BufferError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum PsDeflateError {
    #[error(transparent)]
    BufferError(#[from] BufferError),
    #[error("Decompression error: invalid data")]
    BadData,
    #[error("Insufficient buffer size, output too large")]
    InsufficientSpace,
}

impl From<libdeflater::CompressionError> for PsDeflateError {
    fn from(error: libdeflater::CompressionError) -> Self {
        match error {
            libdeflater::CompressionError::InsufficientSpace => Self::InsufficientSpace,
        }
    }
}

impl From<libdeflater::DecompressionError> for PsDeflateError {
    fn from(error: libdeflater::DecompressionError) -> Self {
        match error {
            libdeflater::DecompressionError::BadData => Self::BadData,
            libdeflater::DecompressionError::InsufficientSpace => Self::InsufficientSpace,
        }
    }
}
