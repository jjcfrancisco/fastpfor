use std::io::Cursor;

use fastpfor::{Composition, Compressor, FastPFOR, FastPForResult, VariableByte};

pub enum Codec {
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
            Codec::VariableByte(vb) => {
                vb.compress(input, input_length, input_offset, output, output_offset)
            }
            Codec::Composition(comp) => {
                comp.compress(input, input_length, input_offset, output, output_offset)
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
            Codec::VariableByte(vb) => {
                vb.uncompress(input, input_length, input_offset, output, output_offset)
            }
            Codec::Composition(comp) => {
                comp.uncompress(input, input_length, input_offset, output, output_offset)
            }
        }
    }
}


pub fn get_codecs() -> Vec<Codec> {
    vec![
        Codec::VariableByte(VariableByte::new()),
        Codec::Composition(Box::new(Composition::new(
            FastPFOR::default(),
            VariableByte::new(),
        ))),
    ]
}
