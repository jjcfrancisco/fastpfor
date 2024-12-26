use std::io::Cursor;

use crate::bytebuffer::ByteBuffer;
use crate::cursor::IncrementCursor;
use crate::helpers::{extract7bits, extract_7bits_maskless};
use crate::{FastPForResult, Output};

#[derive(Debug)]
pub struct VariableByte;

impl VariableByte {
    pub fn compress(
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
                let mut out_pos_tmp = out_pos.position();
                for k in in_pos.position() as i32..(in_pos.position() as i32 + inlength) {
                    let val = input[k as usize] as i64;
                    if val < (1 << 7) {
                        // output.push((val | (1 << 7)) as u8);
                        output[out_pos_tmp as usize] = (val | (1 << 7)) as u8;
                        out_pos_tmp += 1;
                    } else if val < (1 << 14) {
                        output[out_pos_tmp as usize] = extract7bits(0, val) as u8;
                        out_pos_tmp += 1;
                        output[out_pos_tmp as usize] =
                            (extract_7bits_maskless(1, val) | (1 << 7)) as u8;
                        out_pos_tmp += 1;
                    } else if val < (1 << 21) {
                        output[out_pos_tmp as usize] = extract7bits(0, val) as u8;
                        out_pos_tmp += 1;
                        output[out_pos_tmp as usize] = extract7bits(1, val) as u8;
                        out_pos_tmp += 1;
                        output[out_pos_tmp as usize] =
                            (extract_7bits_maskless(2, val) | (1 << 7)) as u8;
                        out_pos_tmp += 1;
                    } else if val < (1 << 28) {
                        output[out_pos_tmp as usize] = extract7bits(0, val) as u8;
                        out_pos_tmp += 1;
                        output[out_pos_tmp as usize] = extract7bits(1, val) as u8;
                        out_pos_tmp += 1;
                        output[out_pos_tmp as usize] = extract7bits(2, val) as u8;
                        out_pos_tmp += 1;
                        output[out_pos_tmp as usize] =
                            (extract_7bits_maskless(3, val) | (1 << 7)) as u8;
                        out_pos_tmp += 1;
                    } else {
                        output[out_pos_tmp as usize] = extract7bits(0, val) as u8;
                        out_pos_tmp += 1;
                        output[out_pos_tmp as usize] = extract7bits(1, val) as u8;
                        out_pos_tmp += 1;
                        output[out_pos_tmp as usize] = extract7bits(2, val) as u8;
                        out_pos_tmp += 1;
                        output[out_pos_tmp as usize] = extract7bits(3, val) as u8;
                        out_pos_tmp += 1;
                        output[out_pos_tmp as usize] =
                            (extract_7bits_maskless(4, val) | (1 << 7)) as u8;
                        out_pos_tmp += 1;
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
        for k in in_pos.position()..(in_pos.position() + inlength as u64) {
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

    pub fn uncompress(
        &mut self,
        input: &Vec<i32>,
        in_pos: &mut Cursor<i32>,
        inlength: i32,
        output: &mut Output,
        out_pos: &mut Cursor<i32>,
    ) -> FastPForResult<()> {
        match output {
            Output::I32(output) => {
                let mut s = 0;
                let mut val = 0;
                let mut p = in_pos.position() as i32;
                let final_p = in_pos.position() as i32 + inlength;
                let mut tmp_outpos = out_pos.position();
                let mut shift = 0;
                let mut v = 0;

                while p < final_p {
                    val = input[p as usize];
                    let c = val >> s;
                    s += 8;
                    p += s >> 5;
                    s &= 31;
                    shift = shift.min(31); // for safety
                    v += (c & 127) << shift;
                    if (c & 128) == 128 {
                        output[tmp_outpos as usize] = v;
                        tmp_outpos += 1;
                        v = 0;
                        shift = 0;
                    } else {
                        shift += 7;
                    }
                }
                out_pos.set_position(tmp_outpos);
                in_pos.add(inlength);

                FastPForResult::Ok(())
            }
            Output::Byte(output) => {
                let mut p = in_pos.position() as i32;
                let final_p = in_pos.position() as i32 + inlength;
                let mut tmp_outpos = out_pos.position();
                let mut v = 0;
                while p < final_p {
                    v = input[p as usize];
                    if input[p as usize] < 0 {
                        p += 1;
                        continue;
                    }
                    v |= (input[p as usize + 1]) << 7;
                    if input[p as usize + 1] < 0 {
                        p += 2;
                        continue;
                    }
                    v |= (input[p as usize + 2]) << 14;
                    if input[p as usize + 2] < 0 {
                        p += 3;
                        continue;
                    }
                    v |= (input[p as usize + 3]) << 21;
                    if input[p as usize + 3] < 0 {
                        p += 4;
                        continue;
                    }
                    v |= (input[p as usize + 4]) << 28;
                    p += 5;
                    output[tmp_outpos as usize] = v as u8;
                    tmp_outpos += 1;
                }
                out_pos.set_position(tmp_outpos);
                in_pos.add(inlength);
                FastPForResult::Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        // All empty
        let empty_array = vec![];
        let mut codec = VariableByte;
        let mut output_compress = Output::I32(vec![]);
        codec
            .compress(
                &empty_array,
                &mut Cursor::new(0),
                0,
                &mut output_compress,
                &mut Cursor::new(0),
            )
            .expect("Failed to compress");
        let comp = {
            match output_compress {
                Output::I32(output) => output,
                _ => panic!("Output is not I32"),
            }
        };
        let mut output_uncompress = Output::I32(vec![]);
        codec
            .uncompress(
                &comp,
                &mut Cursor::new(0),
                0,
                &mut output_uncompress,
                &mut Cursor::new(0),
            )
            .expect("Failed to uncompress");
        let uncomp = {
            match output_uncompress {
                Output::I32(output) => output,
                _ => panic!("Output is not I32"),
            }
        };
        assert_eq!(empty_array, uncomp);
    }
}
