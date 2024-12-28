use std::io::Cursor;

use crate::cursor::IncrementCursor;
use crate::{Codec, IntegerCodec, FastPForResult};

pub struct Composition {
    c1: Codec,
    c2: Codec,
}

impl Composition {
    pub fn new(c1: Codec, c2: Codec) -> Self {
        Composition { c1, c2 }
    }
}

impl Composition {
    fn compress(
        &mut self,
        input: &Vec<i32>,
        input_offset: &mut Cursor<i32>,
        mut input_length: i32,
        output: &mut Vec<i32>,
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let inpos_init = input_offset.position();
        let outpos_init = output_offset.position();
        self.c1.compress(input, input_offset, input_length, output, output_offset)?;
        if output_offset.position() == outpos_init {
            output[outpos_init as usize] = 0;
            output_offset.increment();
        }
        input_length -= input_offset.position() as i32 - inpos_init as i32;
        self.c2.compress(input, input_offset, input_length, output, output_offset)
    }

    fn uncompress(
        &mut self,
        input: &Vec<i32>,
        input_offset: &mut Cursor<i32>,
        mut input_length: i32,
        output: &mut Vec<i32>,
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let final_init = input_offset.position() as i32;
        self.c1.uncompress(input, input_offset, input_length, output, output_offset)?;
        input_length -= input_offset.position() as i32 - final_init;
        self.c2.uncompress(input, input_offset, input_length, output, output_offset)
    }

}
