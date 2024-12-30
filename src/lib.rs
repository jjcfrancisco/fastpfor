mod bitpacking;
mod bytebuffer;
mod composition;
mod compressor;
mod cursor;
mod error;
mod fastpfor;
mod helpers;
mod integer_codec;
mod variable_byte;

pub use composition::Composition;
pub use compressor::Compressor;
pub use error::{FastPForError, FastPForResult};
pub use fastpfor::{FastPFOR, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE};
pub use variable_byte::VariableByte;

#[derive(Debug, PartialEq)]
pub enum Output {
    Byte(Vec<u8>),
    I32(Vec<i32>),
}
