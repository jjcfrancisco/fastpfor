use std::io::Cursor;

use crate::{FastPForResult, Codec};

pub trait Skippable {
    fn headless_compress(
        &mut self,
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()>;
}

impl Skippable for Codec {
    fn headless_compress(
        &mut self,
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFor(fastpfor) => {
                fastpfor.headless_compress(input, input_length, input_offset, output, output_offset)
            }
            Codec::VariableByte(vb) => {
                vb.headless_compress(input, input_length, input_offset, output, output_offset)
            }
        }
    }
}
