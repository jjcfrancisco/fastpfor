pub mod bytebuffer;
mod cursor;
mod error;
mod integer_compression;

pub use error::{FastPForError, FastPForResult};
pub use integer_compression::fastpfor::{
    FastPFOR, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE,
};
pub use integer_compression::variable_byte::VariableByte;
pub use integer_compression::{
    codec::Codec, composition::Composition, integer_codec::Integer, skippable_codec::Skippable,
};

#[derive(Debug, PartialEq)]
pub enum Output {
    Byte(Vec<u8>),
    I32(Vec<i32>),
}
