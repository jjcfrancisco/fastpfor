use crate::bytebuffer::ByteBuffer;
use crate::cursor::IncrementCursor;
use crate::helpers::{extract7bits, extract_7bits_maskless};
use crate::FastPForResult;
use crate::Output;
use std::io::Cursor;

pub struct VariableByte;

impl VariableByte {
    fn compress(
        &mut self,
        input: &Vec<i32>,
        in_pos: &mut Cursor<i32>,
        inlength: i32,
        output: &mut Output,
        out_pos: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        match output {
            Output::I32(output) => self.headless_compress(input, in_pos, inlength, output, out_pos),
            Output::Byte(output) => {
                if inlength == 0 {
                    // Return early if there is no data to compress
                    return Ok(());
                }
                let out_pos_tmp = out_pos.position();
                for k in in_pos.position() as i32..(in_pos.position() as i32 + inlength) {
                    let val = input[k as usize] as i64;
                    if val < (1 << 7) {
                        output.push((val | (1 << 7)) as u8);
                    } else if val < (1 << 14) {
                        output.push(extract7bits(0, val).into());
                        output.push(extract_7bits_maskless(1, val) | (1 << 7));
                    } else if val < (1 << 21) {
                        output.push(extract7bits(0, val));
                        output.push(extract7bits(1, val));
                        output.push(extract_7bits_maskless(2, val) | (1 << 7));
                    } else if val < (1 << 28) {
                        output.push(extract7bits(0, val));
                        output.push(extract7bits(1, val));
                        output.push(extract7bits(2, val));
                        output.push(extract_7bits_maskless(3, val) | (1 << 7));
                    } else {
                        output.push(extract7bits(0, val));
                        output.push(extract7bits(1, val));
                        output.push(extract7bits(2, val));
                        output.push(extract7bits(3, val));
                        output.push(extract_7bits_maskless(4, val) | (1 << 7));
                    }
                }
                out_pos.set_position(out_pos_tmp + inlength as u64);
                in_pos.add(inlength);
                FastPForResult::Ok(())
            }
        }
    }

    fn headless_compress(
        &mut self,
        input: &Vec<i32>,
        in_pos: &mut Cursor<i32>,
        inlength: i32,
        output: &mut Vec<i32>,
        out_pos: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        if inlength == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let mut buf = ByteBuffer::new(inlength * 8);
        for k in in_pos.position() as i32..(in_pos.position() as i32 + inlength) {
            let val = input[k as usize] as i64;
            if val < (1 << 7) {
                buf.put((val | (1 << 7)) as u8);
            } else if val < (1 << 14) {
                buf.put(extract7bits(0, val));
                buf.put(extract_7bits_maskless(1, val) | (1 << 7));
            } else if val < (1 << 21) {
                buf.put(extract7bits(0, val));
                buf.put(extract7bits(1, val));
                buf.put(extract_7bits_maskless(2, val) | (1 << 7));
            } else if val < (1 << 28) {
                buf.put(extract7bits(0, val));
                buf.put(extract7bits(1, val));
                buf.put(extract7bits(2, val));
                buf.put(extract_7bits_maskless(3, val) | (1 << 7));
            } else {
                buf.put(extract7bits(0, val));
                buf.put(extract7bits(1, val));
                buf.put(extract7bits(2, val));
                buf.put(extract7bits(3, val));
                buf.put(extract_7bits_maskless(4, val) | (1 << 7));
            }
        }
        while buf.position() % 4 != 0 {
            buf.put(0);
        }
        let length = buf.position();
        buf.flip();
        let ibuf = buf.as_int_buffer();
        ibuf.get(output, out_pos.position() as usize, (length / 4) as usize);
        out_pos.add((length / 4) as i32);
        in_pos.add(inlength);

        FastPForResult::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        // All empty
        let mut codec = VariableByte;
        let mut output = Output::I32(Vec::new());
        codec
            .compress(
                &vec![],
                &mut Cursor::new(0),
                0,
                &mut output,
                &mut Cursor::new(0),
            )
            .expect("Failed to compress");
        assert_eq!(output, Output::I32(vec![]));
    }
}
