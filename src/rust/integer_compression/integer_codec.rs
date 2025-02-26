use std::io::Cursor;

use crate::rust::{Codec, FastPForResult};

pub trait Integer<T> {
    fn compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [T],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()>;

    fn uncompress(
        &mut self,
        input: &[T],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()>;
}

impl Integer<u32> for Codec {
    fn compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFor(fastpfor) => {
                fastpfor.compress(input, input_length, input_offset, output, output_offset)
            }
            Codec::VariableByte(vb) => {
                vb.compress(input, input_length, input_offset, output, output_offset)
            }
            Codec::JustCopy(jc) => {
                jc.compress(input, input_length, input_offset, output, output_offset)
            }
        }
    }

    fn uncompress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFor(fastpfor) => {
                fastpfor.uncompress(input, input_length, input_offset, output, output_offset)
            }
            Codec::VariableByte(vb) => {
                vb.uncompress(input, input_length, input_offset, output, output_offset)
            }
            Codec::JustCopy(jc) => {
                jc.uncompress(input, input_length, input_offset, output, output_offset)
            }
        }
    }
}
