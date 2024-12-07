use crate::fastpfor::cursor::IncrementCursor;
use crate::{FastPForError, Result};

mod bitpacking;
mod bytebuffer;
mod cursor;
mod helpers;

use std::io::Cursor;

const BLOCK_SIZE: i32 = 256;
#[allow(dead_code)]
const OVERHEAD_OF_EACH_EXCEPT: i32 = 8;
const DEFAULT_PAGE_SIZE: i32 = 65536;
#[allow(dead_code)]
const ZERO_DATA_POINTERS: [i32; 32] = [0; 32];

pub struct FastPFOR {
    pub data_to_be_packed: Vec<Vec<i32>>,
    pub bytes_container: bytebuffer::ByteBuffer,
    pub page_size: i32,
    pub data_pointers: Vec<usize>,
    pub freqs: Vec<i32>,
    pub bestbbestcexceptmaxb: [i32; 3],
}

impl FastPFOR {
    pub fn new(page_size: i32) -> FastPFOR {
        FastPFOR {
            page_size,
            // KEEP FOR REFERENCE RE byte_container
            // Example: Write a u32 value in little-endian
            //pub fn write_to_container(&mut self, value: u32) {
            //    self.byte_container.extend(&value.to_le_bytes());
            //}

            // Example: Read a u32 value in little-endian from a specific offset
            //pub fn read_from_container(&self, offset: usize) -> u32 {
            //    u32::from_le_bytes(
            //        self.byte_container[offset..offset + 4]
            //            .try_into()
            //            .expect("Slice must be 4 bytes long"),
            //    )
            //}
            bytes_container: bytebuffer::ByteBuffer::new(
                (3 * page_size / BLOCK_SIZE + page_size) as usize,
            ),
            data_to_be_packed: {
                let mut data_to_be_packed: Vec<Vec<i32>> = vec![Vec::new(); 33];
                for _ in 1..data_to_be_packed.len() {
                    data_to_be_packed.push(vec![1; DEFAULT_PAGE_SIZE as usize / 32 * 4]);
                }
                data_to_be_packed
            },
            data_pointers: vec![0; 33],
            freqs: vec![0; 33],
            bestbbestcexceptmaxb: [0; 3],
        }
    }

    pub fn compress(
        &mut self,
        input: &mut Vec<i32>,
        in_pos: &mut Cursor<i32>,
        inlength: i32,
        output: &mut Vec<i32>,
        out_pos: &mut Cursor<i32>,
    ) -> Result<()> {
        let inlength = helpers::greatest_multiple(inlength, BLOCK_SIZE as i32);
        if inlength == 0 {
            return Err(FastPForError::Compress("inlength = 0.".to_string()));
        }
        output[out_pos.position() as usize] = inlength;
        out_pos.increment();
        let _inlength = helpers::greatest_multiple(inlength, BLOCK_SIZE as i32);
        let pos = in_pos.position() as i32;
        let final_inpos = pos + _inlength;
        while in_pos.position() as i32 != final_inpos {
            let this_size = std::cmp::min(self.page_size, final_inpos - pos);
            self.encode_page(input, in_pos, this_size, output, out_pos);
        }

        Ok(())
    }

    fn encode_page(
        &mut self,
        input: &mut Vec<i32>,
        in_pos: &mut Cursor<i32>,
        thissize: i32,
        output: &mut Vec<i32>,
        out_pos: &mut Cursor<i32>,
    ) {
        let header_pos = out_pos.position() as usize;
        out_pos.increment();
        let mut tmp_out_pos = out_pos.position();

        // Data pointers to 0
        self.data_pointers.fill(0);
        self.bytes_container.clear();

        let mut tmp_in_pos = in_pos.position() as i32;
        let final_in_pos = tmp_in_pos as i32 + thissize - BLOCK_SIZE;
        while tmp_in_pos <= final_in_pos {
            self.best_b_from_data(input, tmp_in_pos);
            let tmp_best_b = self.bestbbestcexceptmaxb[0];
            self.bytes_container.put(self.bestbbestcexceptmaxb[0] as u8);
            self.bytes_container.put(self.bestbbestcexceptmaxb[1] as u8);
            if self.bestbbestcexceptmaxb[1] > 0 {
                self.bytes_container.put(self.bestbbestcexceptmaxb[2] as u8);
                let index = self.bestbbestcexceptmaxb[2] - self.bestbbestcexceptmaxb[0];
                if self.data_pointers[index as usize] + self.bestbbestcexceptmaxb[1] as usize
                    >= self.data_to_be_packed[index as usize].len()
                {
                    let mut new_size = 2
                        * (self.data_pointers[index as usize]
                            + self.bestbbestcexceptmaxb[1] as usize)
                            as i32;
                    new_size = helpers::greatest_multiple(new_size + 31, 32);
                    self.data_to_be_packed[index as usize].resize(new_size as usize, 1);
                    self.data_to_be_packed[index as usize]
                        .clone_from_slice(&vec![1; new_size as usize]);
                }
                for k in 0..BLOCK_SIZE {
                    if (input[(k + tmp_in_pos) as usize] >> self.bestbbestcexceptmaxb[0]) != 0 {
                        self.bytes_container.put(k as u8);
                        self.data_to_be_packed[index as usize]
                            [self.data_pointers[index as usize]] =
                            input[(k + tmp_in_pos) as usize] >> tmp_best_b;
                    }
                }
            }
            for k in (0..BLOCK_SIZE).step_by(32) {
                bitpacking::fast_pack(
                    input,
                    (tmp_in_pos + k) as usize,
                    output,
                    tmp_out_pos as usize,
                    tmp_best_b as isize,
                );
                tmp_out_pos += tmp_best_b as u64;
            }
            tmp_in_pos += BLOCK_SIZE;
        }
        in_pos.set_position(tmp_in_pos as u64);
        output[header_pos as usize] = tmp_out_pos as i32 - header_pos as i32;
        let byte_size = self.bytes_container.position();
        while (self.bytes_container.position() & 3) != 0 {
            println!("test");
            self.bytes_container.put(0);
        }
        output[tmp_out_pos as usize] = byte_size as i32;
        let how_many_ints = self.bytes_container.position() / 4;
        self.bytes_container.flip();
    }

    fn best_b_from_data(&mut self, input: &mut Vec<i32>, pos: i32) {
        self.freqs.fill(0);
        let k_end = pos + BLOCK_SIZE;
        for k in pos..k_end {
            self.freqs[helpers::bits(input[k as usize])] += 1;
        }

        self.bestbbestcexceptmaxb[0] = 32;
        while self.freqs[self.bestbbestcexceptmaxb[0] as usize] == 0 {
            self.bestbbestcexceptmaxb[0] -= 1;
        }

        let mut bestcost = self.bestbbestcexceptmaxb[0] * BLOCK_SIZE;
        let mut cexcept: i32 = 0;
        self.bestbbestcexceptmaxb[1] = cexcept;

        for b in (0..self.bestbbestcexceptmaxb[0] - 1).rev() {
            cexcept += self.freqs[b as usize + 1];
            if cexcept == BLOCK_SIZE {
                break;
            }
            let mut thiscost = cexcept * OVERHEAD_OF_EACH_EXCEPT
                + cexcept * (self.bestbbestcexceptmaxb[2] - b)
                + b * BLOCK_SIZE
                + 8;
            if self.bestbbestcexceptmaxb[2] - b == 1 {
                thiscost -= cexcept;
            }
            if thiscost < bestcost {
                bestcost = thiscost;
                self.bestbbestcexceptmaxb[0] = b;
                self.bestbbestcexceptmaxb[1] = cexcept;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fastpfor_test() {
        let mut codec1 = FastPFOR::new(DEFAULT_PAGE_SIZE);
        let mut data = vec![0; BLOCK_SIZE as usize];
        data[126] = -1;
        let mut out_buf = vec![0; data.len() * 4];
        let mut in_pos = Cursor::new(0);
        let mut out_pos = Cursor::new(0);
        codec1
            .compress(
                &mut data,
                &mut in_pos,
                BLOCK_SIZE as i32,
                &mut out_buf,
                &mut out_pos,
            )
            .unwrap();
    }

    // #[test]
    // fn basic_example() {
    //     let mut data = vec![0; 2342351];
    //     for k in 0..data.len() {
    //         data[k] = k as i32;
    //     }
    //     let mut compressed = vec![0; data.len() + 1024];
    //     let mut input_offset = Cursor::new(0);
    //     let mut output_offset = Cursor::new(0);
    //     let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE);
    //     let result = codec.compress(
    //         &mut data.clone(),
    //         &mut input_offset,
    //         data.len() as i32,
    //         &mut compressed,
    //         &mut output_offset,
    //     );
    //     if result.is_err() {
    //         println!("{:?}", result.err().unwrap());
    //     }
    // }

    // #[test]
    // fn test_new() {
    //     let mut fastpfor = FastPFOR::new(DEFAULT_PAGE_SIZE);
    //     let mut input = vec![-1];
    //     let input_len = input.len() as i32;
    //     let mut output = vec![0i32; input.len() * 4 * 4];

    //     let mut inpos = Cursor::new(0);
    //     let mut outpos = Cursor::new(0);

    //     fastpfor
    //         .compress(&mut input, &mut inpos, input_len, &mut output, &mut outpos)
    //         .unwrap();
    // }

    // #[test]
    // fn saul_test() {
    //     let mut fastpfor = FastPFOR::new(DEFAULT_PAGE_SIZE);
    //
    //     for i in 0..50 {
    //
    //         let mut a = vec![2, 3, 4, 5];
    //         let mut b = vec![0; 90];
    //         let c = a.len() as i32;
    //
    //         let mut a_offset = Cursor::new(0);
    //         let mut b_offset = Cursor::new(i);
    //
    //         let res = fastpfor
    //             .compress(&mut a, &mut a_offset, c, &mut b, &mut b_offset);
    //         if res.is_err() {
    //             // Printout error message without panic
    //             println!("{:?}", res.err().unwrap());
    //         }
    //     }
    // }

    // #[test]
    // fn all_power_of_two_test() {
    //     let mut fastpfor = FastPFOR::new(DEFAULT_PAGE_SIZE);
    //
    //     for i in 0..50 {
    //
    //         let mut a = vec![1i32 << 42];
    //         let a_size = a.len() as i32;
    //         let mut b = vec![0; 90];
    //
    //         let mut a_offset = Cursor::new(0);
    //         let mut b_offset = Cursor::new(i);
    //
    //         fastpfor
    //             .compress(&mut a, &mut a_offset, a_size, &mut b, &mut b_offset)
    //             .unwrap();
    //     }
    // }
}
