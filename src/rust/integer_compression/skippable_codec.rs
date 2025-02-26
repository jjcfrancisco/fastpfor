use std::io::Cursor;

use crate::rust::{Codec, FastPForResult};

pub trait Skippable {
    fn headless_compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()>;

    fn headless_uncompress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
        num: u32,
    ) -> FastPForResult<()>;
}

impl Skippable for Codec {
    fn headless_compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFor(fastpfor) => {
                fastpfor.headless_compress(input, input_length, input_offset, output, output_offset)
            }
            Codec::VariableByte(vb) => {
                vb.headless_compress(input, input_length, input_offset, output, output_offset)
            }
            Codec::JustCopy(jc) => {
                jc.headless_compress(input, input_length, input_offset, output, output_offset)
            }
        }
    }

    fn headless_uncompress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
        num: u32,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFor(fastpfor) => fastpfor.headless_uncompress(
                input,
                input_length,
                input_offset,
                output,
                output_offset,
                num,
            ),
            Codec::VariableByte(vb) => vb.headless_uncompress(
                input,
                input_length,
                input_offset,
                output,
                output_offset,
                num,
            ),
            Codec::JustCopy(jc) => jc.headless_uncompress(
                input,
                input_length,
                input_offset,
                output,
                output_offset,
                num,
            ),
        }
    }
}
