use crate::cursor::IncrementCursor;
use crate::Codec;
use crate::FastPForResult;
use std::io::Cursor;

pub struct Composition {
    c1: Codec,
    c2: Codec,
}

impl Composition {
    pub fn new(c1: Codec, c2: Codec) -> Self {
        Composition { c1, c2 }
    }

    pub fn compress(
        &mut self,
        input: &[i32],
        input_offset: &mut Cursor<i32>,
        mut input_length: i32,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let inpos_init = input_offset.position();
        let outpos_init = output_offset.position();
        self.c1
            .compress(input, input_length, input_offset, output, output_offset)?;
        if output_offset.position() == outpos_init {
            output[outpos_init as usize] = 0;
            output_offset.increment();
        }
        input_length -= input_offset.position() as i32 - inpos_init as i32;
        self.c2
            .compress(input, input_length, input_offset, output, output_offset)
    }

    pub fn uncompress(
        &mut self,
        input: &[i32],
        input_offset: &mut Cursor<i32>,
        mut input_length: i32,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let final_init = input_offset.position() as i32;
        self.c1
            .uncompress(input, input_length, input_offset, output, output_offset)?;
        input_length -= input_offset.position() as i32 - final_init;
        self.c2
            .uncompress(input, input_length, input_offset, output, output_offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fastpfor::FastPFOR;
    use crate::fastpfor::{BLOCK_SIZE_256, DEFAULT_PAGE_SIZE};
    use crate::variable_byte::VariableByte;

    #[test]
    fn test_composition() {
        let fastpfor = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_256);
        let vb = VariableByte::new();
        let mut comp = Composition::new(Codec::FastPFor(fastpfor), Codec::VariableByte(vb));
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut output = vec![0; 10];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);
        let input_length = 10;
        comp.compress(
            &input,
            &mut input_offset,
            input_length,
            &mut output,
            &mut output_offset,
        ).expect("Failed to compress");
    }
}

