pub type Result<T> = core::result::Result<T, FastPForError>;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FastPForError {
    #[error("Unable to uncompress data: {0}")]
    Uncompress(String),
    #[error("Unable to compress data: {0}")]
    Compress(String),
    #[error("Invalid block size: {0}")]
    InvalidBlockSize(String),
}
