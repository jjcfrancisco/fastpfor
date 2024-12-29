use crate::{Composition, FastPFOR, FastPForResult, VariableByte};
use std::io::Cursor;

pub enum Codec {
    FastPFor(FastPFOR),
    VariableByte(VariableByte),
    Composition(Box<Composition>),
}

impl Codec {
    pub fn compress(
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
            Codec::Composition(comp) => {
                comp.compress(input, input_offset, input_length, output, output_offset)
            }
        }
    }

    pub fn uncompress(
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
            Codec::Composition(comp) => {
                comp.uncompress(input, input_offset, input_length, output, output_offset)
            }
        }
    }
}

