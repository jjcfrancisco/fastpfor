use std::io::Cursor;

use crate::FastPForResult;

pub trait Integer<T> {
    fn compress(
        &mut self,
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [T],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()>;

    fn uncompress(
        &mut self,
        input: &[T],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()>;
}

pub trait Skippable {
    fn headless_compress(
        &mut self,
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()>;
}
