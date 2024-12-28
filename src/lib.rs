mod bitpacking;
mod bytebuffer;
mod composition;
mod cursor;
mod error;
mod fastpfor;
mod helpers;
mod variable_byte;
mod integer_codec;

pub use error::{FastPForError, FastPForResult};
pub use fastpfor::{FastPFOR, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE};
pub use variable_byte::{Compressor, VariableByte};
pub use integer_codec::IntegerCodec;

pub enum Codec {
    VariableByte(VariableByte),
    FastPFOR(FastPFOR),
}

