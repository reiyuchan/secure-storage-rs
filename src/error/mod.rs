#[derive(uniffi::Error)]
#[uniffi(flat_error)]
#[derive(Debug, thiserror::Error)]
pub enum SecureStorageError {
    #[error("ERROR: {0}")]
    Error(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, SecureStorageError>;
