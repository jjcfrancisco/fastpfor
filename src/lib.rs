mod bitpacking;
mod bytebuffer;
mod cursor;
mod error;
mod fastpfor;
mod helpers;
mod variable_byte;

pub use error::{FastPForError, FastPForResult};
pub use fastpfor::{FastPFOR, BLOCK_SIZE_256, BLOCK_SIZE_128, DEFAULT_PAGE_SIZE};
pub use variable_byte::VariableByte;

#[derive(Debug, PartialEq)]
pub enum Output {
    Byte(Vec<u8>),
    I32(Vec<i32>),
}
