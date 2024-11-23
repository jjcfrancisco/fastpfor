use crate::{FastPForError, Result};

mod bitpacking;
mod cursor;
mod helpers;

use std::io::Cursor;

const BLOCK_SIZE: usize = 256;
#[allow(dead_code)]
const OVERHEAD_OF_EACH_EXCEPT: i32 = 8;
#[allow(dead_code)]
const DEFAULT_PAGE_SIZE: i32 = 65536;
const ZERO_DATA_POINTERS: [i32; 32] = [0; 32];

#[allow(dead_code)]
struct FastPFOR {
    data_to_be_packed: Vec<Vec<usize>>,
    byte_container: Vec<u8>,
    page_size: i32,
    data_pointers: Vec<usize>,
    freqs: Vec<usize>,
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
            byte_container: Vec::with_capacity(
                3 * page_size as usize / BLOCK_SIZE as usize + page_size as usize,
            ),
            data_to_be_packed: {
                let mut data_to_be_packed: Vec<Vec<usize>> = vec![Vec::new(); 33];
                for _ in 1..data_to_be_packed.len() {
                    data_to_be_packed.push(vec![1; DEFAULT_PAGE_SIZE as usize / 32 * 4]);
                }
                data_to_be_packed
            },
            data_pointers: vec![0; 33],
            freqs: vec![0; 33],
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
        let inlength = helpers::floor_by(inlength, BLOCK_SIZE as i32);
        if inlength == 0 {
            // Should this be an error?
            // return Err(FastPForError::Compress("inlength = 0.".to_string()));
            return Ok(());
        }
        output[out_pos.position() as usize] = inlength;
        out_pos.set_position(out_pos.position() + 1);
        headless_compress(input, in_pos, inlength, output, out_pos);

        Ok(())
    }
}

fn headless_compress(
    input: &mut Vec<i32>,
    inpos: &mut Cursor<i32>,
    inlength: i32,
    output: &mut Vec<i32>,
    outpos: &mut Cursor<i32>,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fastpfor_test() {
        let mut codec1 = FastPFOR::new(DEFAULT_PAGE_SIZE);
        let mut data = vec![0; BLOCK_SIZE];
        data[126] = -1;
        let mut out_buf = vec![0; data.len() * 4];
        let mut in_pos = Cursor::new(0);
        let mut out_pos = Cursor::new(0);
        codec1.compress(&mut data, &mut in_pos, BLOCK_SIZE as i32, &mut out_buf, &mut out_pos).unwrap();
        
    }

    // #[test]
    // fn test_new() {
    //     let mut fastpfor = FastPFOR::new(DEFAULT_PAGE_SIZE);
    //     let mut input = vec![-1];
    //     let input_len = input.len() as i32;
    //     let mut output = vec![0i32; input.len() * 4 * 4];
    //
    //     let mut inpos = Cursor::new(0);
    //     let mut outpos = Cursor::new(0);
    //
    //     fastpfor
    //         .compress(&mut input, &mut inpos, input_len, &mut output, &mut outpos)
    //         .unwrap();
    // }
    //
    // #[test]
    // fn saul_test() {
    //     let mut fastpfor = FastPFOR::new(DEFAULT_PAGE_SIZE);
    //
    //     for i in 0..50 {
    //
    //         let mut a = vec![2, 3, 4, 5];
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
    //
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
