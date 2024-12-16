use std::io::Cursor;

use crate::fastpfor::cursor::IncrementCursor;
use crate::fastpfor::{bitpacking, bytebuffer, helpers};
use crate::Result;

const BLOCK_SIZE: i32 = 256;
#[allow(dead_code)]
const OVERHEAD_OF_EACH_EXCEPT: i32 = 8;
const DEFAULT_PAGE_SIZE: i32 = 65536;
#[allow(dead_code)]
const ZERO_DATA_POINTERS: [i32; 32] = [0; 32];

#[derive(Debug)]
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
            bytes_container: bytebuffer::ByteBuffer::new(3 * page_size / BLOCK_SIZE + page_size),
            data_to_be_packed: {
                let mut data_to_be_packed: Vec<Vec<i32>> =
                    vec![vec![0; page_size as usize / 32 * 4]; 33];
                for _ in 1..data_to_be_packed.len() {
                    data_to_be_packed.push(vec![0; page_size as usize / 32 * 4]);
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
        input: &Vec<i32>,
        in_pos: &mut Cursor<i32>,
        inlength: i32,
        output: &mut Vec<i32>,
        out_pos: &mut Cursor<i32>,
    ) -> Result<()> {
        let inlength = helpers::greatest_multiple(inlength, BLOCK_SIZE as i32);
        if inlength == 0 {
            // Return early if there is no data to compress
            return Ok(());
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
        input: &Vec<i32>,
        in_pos: &mut Cursor<i32>,
        thissize: i32,
        output: &mut Vec<i32>,
        out_pos: &mut Cursor<i32>,
    ) {
        let header_pos = out_pos.position() as usize;
        out_pos.increment();
        let mut tmp_out_pos = out_pos.position() as i32;

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
                        self.data_pointers[index as usize] += 1;
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
                tmp_out_pos += tmp_best_b;
            }
            tmp_in_pos += BLOCK_SIZE;
        }
        in_pos.set_position(tmp_in_pos as u64);
        output[header_pos as usize] = tmp_out_pos as i32 - header_pos as i32;
        let byte_size = self.bytes_container.position();
        while (self.bytes_container.position() & 3) != 0 {
            self.bytes_container.put(0);
        }
        // Output should have 3 position as 4
        output[tmp_out_pos as usize] = byte_size as i32;
        tmp_out_pos += 1;
        let how_many_ints = self.bytes_container.position() / 4;
        self.bytes_container.flip();

        self.bytes_container.as_int_buffer().get(
            output,
            tmp_out_pos as usize,
            how_many_ints as usize,
        );
        tmp_out_pos += how_many_ints;
        let mut bitmap = 0;
        for k in 2..=32 {
            if self.data_pointers[k] != 0 {
                bitmap |= 1 << (k - 1);
            }
        }
        output[tmp_out_pos as usize] = bitmap;
        tmp_out_pos += 1;

        for k in 2..=32 {
            if self.data_pointers[k] != 0 {
                output[tmp_out_pos as usize] = self.data_pointers[k] as i32;
                tmp_out_pos += 1;
                let mut j = 0;
                while j < self.data_pointers[k as usize] {
                    // println!("data_to_be_packed[k]: {:?}", self.data_to_be_packed[k as usize]);
                    bitpacking::fast_pack(
                        &self.data_to_be_packed[k as usize],
                        j,
                        output,
                        tmp_out_pos as usize,
                        k as isize,
                    );
                    tmp_out_pos += k as i32;
                    j += 32;
                }

                // Overflow adjustment
                let overflow = j as i32 - self.data_pointers[k as usize] as i32;
                tmp_out_pos -= (overflow * k as i32) / 32;
            }
        }
        out_pos.set_position(tmp_out_pos as u64);
    }

    fn best_b_from_data(&mut self, input: &Vec<i32>, pos: i32) {
        self.freqs.fill(0);
        let k_end = pos + BLOCK_SIZE;
        for k in pos..k_end {
            self.freqs[helpers::bits(input[k as usize])] += 1;
        }

        self.bestbbestcexceptmaxb[0] = 32;
        while self.freqs[self.bestbbestcexceptmaxb[0] as usize] == 0 {
            self.bestbbestcexceptmaxb[0] -= 1;
        }
        self.bestbbestcexceptmaxb[2] = self.bestbbestcexceptmaxb[0];

        let mut bestcost = self.bestbbestcexceptmaxb[0] * BLOCK_SIZE;
        let mut cexcept: i32 = 0;
        self.bestbbestcexceptmaxb[1] = cexcept;

        for b in (0..self.bestbbestcexceptmaxb[0]).rev() {
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

    pub fn uncompress(
        &mut self,
        input: &Vec<i32>,
        in_pos: &mut Cursor<i32>,
        inlength: i32,
        output: &mut Vec<i32>,
        out_pos: &mut Cursor<i32>,
    ) -> Result<()> {
        if inlength == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let outlength = input[in_pos.position() as usize];
        in_pos.increment();
        let mynvalue = helpers::greatest_multiple(outlength, BLOCK_SIZE);
        let final_out = out_pos.position() as i32 + mynvalue;
        while out_pos.position() as i32 != final_out {
            let this_size = std::cmp::min(self.page_size, final_out - out_pos.position() as i32);
            self.decode_page(input, in_pos, output, out_pos, this_size);
        }
        Ok(())
    }

    fn decode_page(
        &mut self,
        input: &Vec<i32>,
        in_pos: &mut Cursor<i32>,
        output: &mut Vec<i32>,
        out_pos: &mut Cursor<i32>,
        thissize: i32,
    ) {
        let init_pos = in_pos.position() as i32;
        let where_meta = input[in_pos.position() as usize];
        in_pos.increment();
        let mut inexcept = init_pos + where_meta;
        let bytesize = input[inexcept as usize];
        inexcept += 1;
        self.bytes_container.clear();
        self.bytes_container.buffer =
            self.bytes_container
                .as_int_buffer()
                .put(input, inexcept as usize, (bytesize + 3) / 4);
        inexcept += (bytesize + 3) / 4;

        let bitmap = input[inexcept as usize];
        inexcept += 1;

        for k in 2..=32 {
            if (bitmap & (1 << (k - 1))) != 0 {
                let size = input[inexcept as usize];
                inexcept += 1;
                let rounded_up = helpers::greatest_multiple(size + 31, 32);
                if self.data_to_be_packed[k as usize].len() < rounded_up as usize {
                    self.data_to_be_packed[k as usize] = vec![0; rounded_up as usize];
                }
                if inexcept + rounded_up / 32 * k <= input.len() as i32 {
                    let mut j = 0;
                    while j < size {
                        bitpacking::fast_unpack(
                            input,
                            inexcept as usize,
                            &mut self.data_to_be_packed[k as usize],
                            j as usize,
                            k as isize,
                        );
                        inexcept += k;
                        j += 32;
                    }
                    let overflow = j - size;
                    inexcept -= (overflow * k) / 32;
                } else {
                    let mut j = 0;
                    let mut buf = vec![0; rounded_up as usize / 32 * k as usize];
                    let init_inexcept = inexcept;
                    // Ensure length is the same as the buffer
                    let length = input.len() - init_inexcept as usize;
                    buf[..length].copy_from_slice(&input[init_inexcept as usize..]);
                    while j < size {
                        bitpacking::fast_unpack(
                            &buf,
                            (inexcept - init_inexcept) as usize,
                            &mut self.data_to_be_packed[k as usize],
                            j as usize,
                            k as isize,
                        );
                        inexcept += k;
                        j += 32;
                    }
                    let overflow = j - size;
                    inexcept -= (overflow * k) / 32;
                }
            }
        }

        self.data_pointers.fill(0);
        let mut tmp_out_pos = out_pos.position() as i32;
        let mut tmp_in_pos = in_pos.position() as i32;

        let run_end = thissize / BLOCK_SIZE;
        for _ in 0..run_end {
            let b = self.bytes_container.get() as i32;
            let cexcept = self.bytes_container.get() & 0xFF;
            for k in (0..BLOCK_SIZE).step_by(32) {
                bitpacking::fast_unpack(
                    input,
                    tmp_in_pos as usize,
                    output,
                    (tmp_out_pos + k) as usize,
                    b as isize,
                );
                tmp_in_pos += b;
            }
            if cexcept > 0 {
                let maxbits = self.bytes_container.get() as i32;
                let index = maxbits - b;
                if index == 1 {
                    for _ in 0..cexcept {
                        let pos = self.bytes_container.get() & 0xFF;
                        output[pos as usize + tmp_out_pos as usize] |= 1 << b;
                    }
                } else {
                    for _ in 0..cexcept {
                        let pos = self.bytes_container.get() & 0xFF;
                        let except_value = self.data_to_be_packed[index as usize]
                            [self.data_pointers[index as usize]];
                        output[pos as usize + tmp_out_pos as usize] |= except_value << b;
                        self.data_pointers[index as usize] += 1;
                    }
                }
            }
            tmp_out_pos += BLOCK_SIZE;
        }
        out_pos.set_position(tmp_out_pos as u64);
        in_pos.set_position(inexcept as u64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fastpfor_test() {
        let mut codec1 = FastPFOR::new(DEFAULT_PAGE_SIZE);
        let mut codec2 = FastPFOR::new(DEFAULT_PAGE_SIZE);
        let mut data = vec![0; BLOCK_SIZE as usize];
        data[126] = -1;
        let mut out_buf = vec![0; data.len() * 4];
        let mut in_pos = Cursor::new(0);
        let mut out_pos = Cursor::new(0);
        codec1
            .compress(
                &data,
                &mut in_pos,
                data.len() as i32,
                &mut out_buf,
                &mut out_pos,
            )
            .unwrap();
        let comp = out_buf[..out_pos.position() as usize].to_vec();
        let initial_values = vec![256, 1, 4, 2116026624, -2147483648, 1, -1];
        let zeros_count = 1024 - initial_values.len();
        let mut out_buf_compressed: Vec<i32> = initial_values;
        out_buf_compressed.extend(vec![0; zeros_count]);
        assert_eq!(out_buf_compressed, out_buf);

        let mut out_buf_uncomp = vec![0; data.len() * 4];
        let mut in_pos_uncomp = Cursor::new(0);
        let mut out_pos_uncomp = Cursor::new(0);
        codec2
            .uncompress(
                &comp,
                &mut in_pos_uncomp,
                comp.len() as i32,
                &mut out_buf_uncomp,
                &mut out_pos_uncomp,
            )
            .unwrap();
        let answer = out_buf_uncomp[..out_pos_uncomp.position() as usize].to_vec();

        for k in 0..BLOCK_SIZE {
            if answer[k as usize] != data[k as usize] {
                panic!("bug {} {} != {}", k, answer[k as usize], data[k as usize]);
            }
        }
    }

    #[test]
    fn test_spurious() {
        let mut c = FastPFOR::new(DEFAULT_PAGE_SIZE);
        let x = vec![0; 1024];
        let mut y = vec![0; 0];
        let mut i0 = Cursor::new(0);
        let mut i1 = Cursor::new(0);
        for inlength in 0..32 {
            c.compress(&x, &mut i0, inlength, &mut y, &mut i1).unwrap();
            assert_eq!(0, i1.position());
        }
    }

    #[test]
    fn test_zero_in_zero_out() {
        let mut c = FastPFOR::new(DEFAULT_PAGE_SIZE);
        let x = vec![0; 0];
        let mut y = vec![0; 0];
        let mut i0 = Cursor::new(0);
        let mut i1 = Cursor::new(0);
        c.compress(&x, &mut i0, 0, &mut y, &mut i1).unwrap();
        assert_eq!(0, i1.position());

        // Needs uncompress
        let mut out = vec![0; 0];
        let mut outpos = Cursor::new(0);
        c.uncompress(&y, &mut i1, 0, &mut out, &mut outpos).unwrap();
        assert_eq!(0, outpos.position());
    }
}
