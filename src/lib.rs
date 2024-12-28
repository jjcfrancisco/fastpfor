mod bitpacking;
mod bytebuffer;
mod composition;
mod cursor;
mod error;
mod fastpfor;
mod helpers;
mod variable_byte;

pub use error::{FastPForError, FastPForResult};
pub use fastpfor::{FastPFOR, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE};
pub use variable_byte::{Compressor, VariableByte};

pub enum Codec {
    VariableByte(VariableByte),
}
