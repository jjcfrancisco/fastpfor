use std::io::Cursor;

use crate::{FastPForResult, Integer, Skippable};

#[derive(Debug)]
pub struct JustCopy;

impl JustCopy {
    pub fn new() -> Self {
        JustCopy
    }
}

impl Default for JustCopy {
    fn default() -> Self {
        JustCopy::new()
    }
}

impl Skippable for JustCopy {
    fn headless_compress(
        &mut self,
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        output[output_offset.position() as usize..]
            .copy_from_slice(&input[input_offset.position() as usize..]);
        input_offset.set_position(input_length as u64);
        output_offset.set_position(input_length as u64);
        FastPForResult::Ok(())
    }

    fn headless_uncompress(
        &mut self,
        input: &[i32],
        #[expect(unused)] input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
        num: i32,
    ) -> FastPForResult<()> {
        output[output_offset.position() as usize..]
            .copy_from_slice(&input[input_offset.position() as usize..]);
        input_offset.set_position(num as u64);
        input_offset.set_position(num as u64);
        output_offset.set_position(num as u64);
        FastPForResult::Ok(())
    }
}

impl Integer<i32> for JustCopy {
    fn compress(
        &mut self,
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        self.headless_compress(input, input_length, input_offset, output, output_offset)
    }

    fn uncompress(
        &mut self,
        input: &[i32],
        input_length: i32,
        input_offset: &mut Cursor<i32>,
        output: &mut [i32],
        output_offset: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        output[output_offset.position() as usize..]
            .copy_from_slice(&input[input_offset.position() as usize..]);
        input_offset.set_position(input_length as u64);
        output_offset.set_position(input_length as u64);
        FastPForResult::Ok(())
    }
}

// @Override
// public void compress(int[] in, IntWrapper inpos, int inlength,
//         int[] out, IntWrapper outpos) {
//     headlessCompress(in,inpos,inlength,out,outpos);
// }
//
// @Override
// public void uncompress(int[] in, IntWrapper inpos, int inlength,
//         int[] out, IntWrapper outpos) {
//     headlessUncompress(in,inpos,inlength,out,outpos,inlength);
// }

// @Override
// public String toString() {
//         return this.getClass().getSimpleName();
// }
//
// @Override
// public void headlessCompress(int[] in, IntWrapper inpos, int inlength,
//         int[] out, IntWrapper outpos) {
//         System.arraycopy(in, inpos.get(), out, outpos.get(), inlength);
//         inpos.add(inlength);
//         outpos.add(inlength);
// }
//
// @Override
// public void headlessUncompress(int[] in, IntWrapper inpos, int inlength,
//         int[] out, IntWrapper outpos, int num) {
//     System.arraycopy(in, inpos.get(), out, outpos.get(), num);
//     inpos.add(num);
//     outpos.add(num);
//
// }
//
