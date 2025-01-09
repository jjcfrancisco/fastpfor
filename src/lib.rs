pub mod bytebuffer;
mod cursor;
mod error;
mod integer_compression;

pub use error::{FastPForError, FastPForResult};
pub use integer_compression::composition::Composition;
pub use integer_compression::compressors::Integer;
pub use integer_compression::fastpfor::{
    FastPFOR, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE,
};
pub use integer_compression::variable_byte::VariableByte;

#[derive(Debug, PartialEq)]
pub enum Output {
    Byte(Vec<u8>),
    I32(Vec<i32>),
}
