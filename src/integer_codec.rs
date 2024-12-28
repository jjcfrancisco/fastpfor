use std::io::Cursor;

use crate::{Codec, Compressor, FastPForResult};

// Define a trait
pub trait IntegerCodec {
    fn compress(
        &mut self,
        input: &Vec<i32>,
        input_offset: &mut Cursor<i32>,
        input_length: i32,
        output: &mut Vec<i32>,
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()>;
    fn uncompress(
        &mut self,
        input: &Vec<i32>,
        input_offset: &mut Cursor<i32>,
        input_length: i32,
        output: &mut Vec<i32>,
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()>;
}

impl IntegerCodec for Codec {
    fn compress(
        &mut self,
        input: &Vec<i32>,
        input_offset: &mut Cursor<i32>,
        input_length: i32,
        output: &mut Vec<i32>,
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::VariableByte(codec) => {
                codec.compress(input, input_offset, input_length, output, output_offset)
            }
            Codec::FastPFOR(codec) => {
                codec.compress(input, input_offset, input_length, output, output_offset)
            }
        }
    }

    fn uncompress(
        &mut self,
        input: &Vec<i32>,
        input_offset: &mut Cursor<i32>,
        input_length: i32,
        output: &mut Vec<i32>,
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::VariableByte(codec) => {
                codec.uncompress(input, input_offset, input_length, output, output_offset)
            }
            Codec::FastPFOR(codec) => {
                codec.uncompress(input, input_offset, input_length, output, output_offset)
            }
        }
    }
}
