use crate::{FastPFOR, FastPForResult, VariableByte, Compressor};
use std::io::Cursor;

pub enum IntegerCodec {
    FastPFor(FastPFOR),
    VariableByte(VariableByte),
}

impl From<FastPFOR> for IntegerCodec {
    fn from(fastpfor: FastPFOR) -> Self {
        IntegerCodec::FastPFor(fastpfor)
    }
}

impl From<VariableByte> for IntegerCodec {
    fn from(vb: VariableByte) -> Self {
        IntegerCodec::VariableByte(vb)
    }
}

impl Compressor<i32> for IntegerCodec {
    fn compress(
        &mut self,
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        match self {
            IntegerCodec::FastPFor(fastpfor) => {
                fastpfor.compress(input, input_length, input_offset, output, output_offset)
            }
            IntegerCodec::VariableByte(vb) => {
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
            IntegerCodec::FastPFor(fastpfor) => {
                fastpfor.uncompress(input, input_length, input_offset, output, output_offset)
            }
            IntegerCodec::VariableByte(vb) => {
                vb.uncompress(input, input_length, input_offset, output, output_offset)
            }
        }
    }
}

