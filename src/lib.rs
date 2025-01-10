pub mod bytebuffer;
mod cursor;
mod error;
mod integer_compression;

pub use error::{FastPForError, FastPForResult};
pub use integer_compression::{
    codec::Codec,
    composition::Composition,
    fastpfor::{FastPFOR, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE},
    integer_codec::Integer,
    just_copy::JustCopy,
    skippable_codec::Skippable,
    variable_byte::VariableByte,
};

#[derive(Debug, PartialEq)]
pub enum Output {
    Byte(Vec<u8>),
    I32(Vec<i32>),
}
