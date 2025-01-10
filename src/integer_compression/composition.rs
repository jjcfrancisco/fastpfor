use std::io::Cursor;

use crate::cursor::IncrementCursor;
use crate::{Codec, FastPForResult, Integer};

pub struct Composition {
    c1: Codec,
    c2: Codec,
}

impl Composition {
    pub fn new<C1, C2>(c1: C1, c2: C2) -> Self
    where
        C1: Into<Codec>,
        C2: Into<Codec>,
    {
        Composition {
            c1: c1.into(),
            c2: c2.into(),
        }
    }

    pub fn compress(
        &mut self,
        input: &[i32],
        mut input_length: i32,
        input_offset: &mut Cursor<i32>,
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
        mut input_length: i32,
        input_offset: &mut Cursor<i32>,
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
    use crate::integer_compression::fastpfor::FastPFOR;
    use crate::integer_compression::variable_byte::VariableByte;

    #[test]
    fn test_composition() {
        let fastpfor = FastPFOR::default();
        let vb = VariableByte::new();
        let mut comp = Composition::new(fastpfor, vb);
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut output = vec![0; 10];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);
        let input_length = 10;
        comp.compress(
            &input,
            input_length,
            &mut input_offset,
            &mut output,
            &mut output_offset,
        )
        .expect("Failed to compress");
    }
}
