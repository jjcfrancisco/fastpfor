use crate::FastPForResult;
use std::io::Cursor;

pub trait IntegerCodec {
    fn compress(
        &mut self,
        input: &Vec<i32>,
        in_pos: &mut Cursor<i32>,
        inlength: i32,
        output: &mut Vec<i32>,
        out_pos: &mut Cursor<i32>,
    ) -> FastPForResult<()>;
}

pub trait ByteIntegerCodec {
    fn compress(
        &mut self,
        input: &Vec<i32>,
        in_pos: &mut Cursor<i32>,
        inlength: i32,
        output: &mut Vec<u8>,
        out_pos: &mut Cursor<i32>,
    ) -> FastPForResult<()>;
}

pub trait SkippableIntegerCodec {
    fn headless_compress(
        &mut self,
        input: &Vec<i32>,
        in_pos: &mut Cursor<i32>,
        inlength: i32,
        output: &mut Vec<i32>,
        out_pos: &mut Cursor<i32>,
    ) -> FastPForResult<()>;
}


