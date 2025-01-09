use std::io::Cursor;

use crate::{FastPForResult, Codec};

pub trait Integer<T> {
    fn compress(
        &mut self,
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [T],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()>;

    fn uncompress(
        &mut self,
        input: &[T],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()>;
}

impl Integer<i32> for Codec {
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
