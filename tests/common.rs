use fastpfor::{FastPFOR, Output, VariableByte, FastPForResult};
use std::io::Cursor;

#[derive(Debug)]
pub enum Codec {
    FastPFOR(FastPFOR),
    VariableByte(VariableByte),
}

impl Codec {
    pub fn compress(
        &mut self,
        input: &Vec<i32>,
        input_offset: &mut Cursor<i32>,
        input_length: i32,
        output: &mut Output,
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFOR(codec) => {
                let outbuf = match output {
                    Output::I32(o) => o,
                    _ => panic!("Output is not I32"),
                };
                codec.compress(input, input_offset, input_length, outbuf, output_offset)
            }
            Codec::VariableByte(codec) => {
                codec.compress(input, input_offset, input_length, output, output_offset)
            }
        }
    }

    pub fn uncompress(
        &mut self,
        input: &Vec<i32>,
        input_offset: &mut Cursor<i32>,
        input_length: i32,
        output: &mut Output,
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFOR(codec) => {
                let outbuf = match output {
                    Output::I32(o) => o,
                    _ => panic!("Output is not I32"),
                };
                codec.uncompress(input, input_offset, input_length, outbuf, output_offset)
            }
            Codec::VariableByte(codec) => {
                codec.uncompress(input, input_offset, input_length, output, output_offset)
            }
        }
    }
}

pub fn get_codecs() -> Vec<Codec> {
    let variablebyte = VariableByte;

    vec![
        Codec::VariableByte(variablebyte),
    ]
}
