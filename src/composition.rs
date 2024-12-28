use std::io::Cursor;

use crate::cursor::IncrementCursor;
use crate::variable_byte::Compressor;
use crate::{Codec, FastPForResult};

pub struct Composition {
    c1: Codec,
    c2: Codec,
}

impl Composition {
    pub fn add(&mut self, c1: Codec, c2: Codec) {
        self.c1 = c1;
        self.c2 = c2;
    }
    pub fn compress(
        &mut self,
        input: &Vec<i32>,
        input_offset: &mut Cursor<i32>,
        mut input_length: i32,
        output: &mut Vec<i32>,
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        let c1 = match &mut self.c1 {
            Codec::VariableByte(c) => c,
        };
        if input_length == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let inpos_init = input_offset.position();
        let outpos_init = output_offset.position();
        c1.compress(input, input_offset, input_length, output, output_offset)?;
        if output_offset.position() == outpos_init {
            output[outpos_init as usize] = 0;
            output_offset.increment();
        }
        input_length -= input_offset.position() as i32 - inpos_init as i32;
        let c2 = match &mut self.c2 {
            Codec::VariableByte(c) => c,
        };
        c2.compress(input, input_offset, input_length, output, output_offset)
    }

    pub fn uncompress(
        &mut self,
        input: &Vec<i32>,
        input_offset: &mut Cursor<i32>,
        mut input_length: i32,
        output: &mut Vec<i32>,
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        let c1 = match &mut self.c1 {
            Codec::VariableByte(c) => c,
        };
        if input_length == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let final_init = input_offset.position() as i32;
        c1.uncompress(input, input_offset, input_length, output, output_offset)?;
        // Java:
        // inlength -= inpos.get() - init;
        input_length -= input_offset.position() as i32 - final_init;
        let c2 = match &mut self.c2 {
            Codec::VariableByte(c) => c,
        };
        c2.uncompress(input, input_offset, input_length, output, output_offset)
    }
}
