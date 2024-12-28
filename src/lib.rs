mod bitpacking;
mod bytebuffer;
mod composition;
mod compressor;
mod cursor;
mod error;
mod fastpfor;
mod helpers;
mod variable_byte;

use std::io::Cursor;

pub use error::{FastPForError, FastPForResult};
pub use fastpfor::{FastPFOR, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE};
pub use variable_byte::VariableByte;

pub enum Codec {
    FastPFor(FastPFOR),
    VariableByte(VariableByte),
}

impl Codec {
    fn compress(
        &mut self,
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFor(fastpfor) => {
                fastpfor.compress(input, input_length, input_offset, output, output_offset)
            }
            Codec::VariableByte(vb) => {
                vb.compress(input, input_length, input_offset, output, output_offset)
            }
        }
    }

    fn uncompress(
        &mut self,
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFor(fastpfor) => {
                fastpfor.uncompress(input, input_length, input_offset, output, output_offset)
            }
            Codec::VariableByte(vb) => {
                vb.uncompress(input, input_length, input_offset, output, output_offset)
            }
        }
    }
}
