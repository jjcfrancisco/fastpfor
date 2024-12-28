use crate::FastPForResult;
use std::io::Cursor;

pub trait Compressor {
    fn compress(
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [Self],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()>
    where
        Self: Sized;

    fn uncompress(
        input: &[Self],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()>
    where
        Self: Sized;
}

