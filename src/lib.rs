mod bitpacking;
mod bytebuffer;
mod cursor;
mod error;
mod fastpfor;
mod helpers;
mod variable_byte;
mod compressor;
mod composition;
mod integer_codec;

pub use error::{FastPForError, FastPForResult};
pub use fastpfor::{FastPFOR, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE};
pub use variable_byte::VariableByte;
pub use composition::Composition;
pub use compressor::Compressor;

#[derive(Debug, PartialEq)]
pub enum Output {
    Byte(Vec<u8>),
    I32(Vec<i32>),
}
