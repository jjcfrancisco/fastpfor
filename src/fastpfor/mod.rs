use crate::{FastPForError, Result};

mod bitpacking;
mod cursor;
mod helpers;

use bytebuffer::ByteBuffer;
use cursor::Cursor;
use zerocopy;

const BLOCK_SIZE: i32 = 128;
#[allow(dead_code)]
const OVERHEAD_OF_EACH_EXCEPT: i32 = 8;
#[allow(dead_code)]
const DEFAULT_PAGE_SIZE: i32 = 65536;
const ZERO_DATA_POINTERS: [i32; 32] = [0; 32];

#[allow(dead_code)]
struct FastPFOR {
    data_to_be_packed: Vec<Vec<i32>>,
    byte_container: Vec<u8>,
    page_size: i32,
    data_pointers: Vec<i32>,
    freqs: Vec<i32>,
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
                let mut data_to_be_packed: Vec<Vec<i32>> = vec![Vec::new(); 33];
                for _ in 1..data_to_be_packed.len() {
                    // data_to_be_packed.push(vec![1; DEFAULT_PAGE_SIZE as usize / 32 * 4]);
                }
                data_to_be_packed
            },
            data_pointers: vec![0; 33],
            freqs: vec![0; 33],
        }
    }

    pub fn compress(
        &mut self,
        input: &mut [i32],
        inpos: &mut cursor::Cursor,
        inlength: i32,
        output: &mut [i32],
        outpos: &mut cursor::Cursor,
    ) -> Result<()> {
        let inlength = helpers::floor_by(inlength, BLOCK_SIZE);
        if inlength == 0 {
            return Err(FastPForError::Compress(
                "inlength = 0. No work done.".to_string(),
            ));
        }
        output[outpos.get() as usize] = inlength;
        outpos.increment();

        self.data_pointers.clone_from_slice(&ZERO_DATA_POINTERS);
        self.freqs.clone_from_slice(&ZERO_DATA_POINTERS);

        let final_inpos = inpos.get() + inlength;

        while inpos.get() != final_inpos {
            let thissize = std::cmp::min(self.page_size, final_inpos - inpos.get()) as isize;
            // TODO: Implement Result so we can return an error here like in the Golang code
            // if err := this.encodePage(in, inpos, thissize, out, outpos); err != nil {
            // 	return errors.New("fastpfor/Compress: " + err.Error())
            // }
            self.encode_page(input, inpos, thissize, output, outpos)?;
        }

        Ok(())
    }

    pub fn uncompress(
        &mut self,
        input: &mut [i32],
        inpos: &mut cursor::Cursor,
        inlength: i32,
        output: &mut [i32],
        outpos: &mut cursor::Cursor,
    ) -> Result<()> {
        if inlength == 0 {
            return Err(FastPForError::Uncompress(
                "inlength = 0. No work done.".to_string(),
            ));
        }

        let nvalue = input[inpos.value as usize];

        self.data_pointers.clone_from_slice(&ZERO_DATA_POINTERS);

        let finalout = outpos.get() + nvalue as i32;
        while outpos.get() != finalout {
            let thissize = std::cmp::min(self.page_size, finalout - outpos.get()) as isize;
            // TODO: Implement Result so we can return an error here like in the Golang code
            // if err := this.decodePage(in, inpos, out, outpos, thissize); err != nil {
            // 	return errors.New("fastpfor/Uncompress: " + err.Error())
            // }
            self.decode_page(input, inpos, output, outpos, thissize)?;
        }

        Ok(())
    }

    fn encode_page(
        &mut self,
        input: &mut [i32],
        inpos: &mut Cursor,
        thissize: isize,
        output: &mut [i32],
        outpos: &mut Cursor,
    ) -> Result<()> {
        let headerpos = outpos.get();
        outpos.increment();
        let mut tmpoutpos = outpos.get();

        self.data_pointers.clone_from_slice(&ZERO_DATA_POINTERS);
        self.byte_container.clear();

        let mut tmpinpos = inpos.get();

        let final_inpos = tmpinpos + thissize as i32 - BLOCK_SIZE;
        while tmpinpos <= final_inpos {
            let (bestb, bestc, maxb) = self.get_best_b_from_data(&input[tmpinpos as usize..]);
            let tmpbestb = bestb;
            self.byte_container.write_u8(bestb as u8);
            self.byte_container.write_u8(bestc as u8);

            if bestc > 0 {
                self.byte_container.write_u8(maxb as u8);
                let index = maxb - bestb;
                if self.data_pointers[index as usize] + bestc as i32
                    >= self.data_to_be_packed[index as usize].len() as i32
                {
                    let new_size = 2 * (self.data_pointers[index as usize] + bestc);
                    let new_size = helpers::ceil_by(new_size, 32);
                    let new_slice = vec![index as i32; new_size as usize];
                    self.data_to_be_packed[index as usize] = new_slice;
                }

                for k in 0..BLOCK_SIZE {
                    if input[(k + tmpinpos) as usize] >> bestb != 0 {
                        self.byte_container.write_u8(k as u8);
                        self.data_to_be_packed[index as usize]
                            [self.data_pointers[index as usize] as usize] =
                            input[(k + tmpinpos) as usize] >> tmpbestb;
                        self.data_pointers[index as usize] += 1;
                    }
                }
            }

            for k in (0..128).step_by(32) {
                bitpacking::fast_pack(
                    input,
                    (tmpinpos + k) as usize,
                    output,
                    tmpoutpos as usize,
                    tmpbestb as isize,
                );
                tmpoutpos += tmpbestb;
            }

            tmpinpos += BLOCK_SIZE;
        }

        // Golang:
        // inpos.Set(int(tmpinpos))
        // out[headerpos] = tmpoutpos - headerpos
        // bytesize := int32(this.byteContainer.Position())
        // for this.byteContainer.Position()&3 != 0 {
        //     this.byteContainer.Put(0)
        // }

        // out[tmpoutpos] = bytesize
        // tmpoutpos += 1
        // howmanyints := (bytesize + 3) / 4
        // this.byteContainer.Flip()
        // this.byteContainer.AsInt32Buffer().GetInt32s(out, int(tmpoutpos), int(howmanyints))
        // tmpoutpos += howmanyints

        // bitmap := int32(0)
        // for k := 1; k <= 32; k++ {
        //     v := this.dataPointers[k]
        //     if v != 0 {
        //         bitmap |= (1 << uint(k-1))
        //     }
        // }

        // out[tmpoutpos] = bitmap
        // tmpoutpos += 1

        inpos.set(tmpinpos as i32);
        output[headerpos as usize] = tmpoutpos - headerpos;
        let bytesize = self.byte_container.get_wpos() as i32;
        while self.byte_container.get_wpos() & 3 != 0 {
            self.byte_container.write_u8(0);
        }

        output[tmpoutpos as usize] = bytesize;
        tmpoutpos += 1;
        let howmanyints = (bytesize + 3) / 4;
        self.byte_container.set_rpos(0);
        self.byte_container.write_i32(howmanyints);

        Ok(())
    }

    fn decode_page(
        &mut self,
        input: &mut [i32],
        inpos: &mut Cursor,
        output: &mut [i32],
        outpos: &mut Cursor,
        size: isize,
    ) -> Result<()> {
        let initpos = inpos.get();
        let wheremeta = input[initpos as usize];
        inpos.increment();

        let mut inexcept = initpos + wheremeta;
        let byte_size = input[inexcept as usize] as usize;
        inexcept = inexcept + 1;
        let my_byte_array = &input[initpos as usize..].to_vec();

        let mut mybp = 0;
        inexcept += ((byte_size + 3) / 4) as i32;
        let bitmap = input[inexcept as usize];
        inexcept += 1;

        for k in 1..33 {
            if bitmap & (1 << (k - 1)) != 0 {
                let size = input[inexcept as usize];
                inexcept += 1;

                if self.data_to_be_packed[k].len() < size as usize {
                    self.data_to_be_packed[k] = vec![k as i32; size as usize];
                }

                for j in 0..size {
                    bitpacking::fast_unpack(
                        input,
                        inexcept as usize,
                        &mut self.data_to_be_packed[k],
                        j as usize,
                        k as isize,
                    );
                    inexcept += k as i32;
                }
            }
        }

        self.data_pointers.clone_from_slice(&ZERO_DATA_POINTERS);
        let mut tmpoutpos = outpos.get() as u32;
        let mut tmpinpos = inpos.get() as u32;

        let mut run = 0;
        let run_end = size / BLOCK_SIZE as isize;

        while run < run_end {
            let bestb = helpers::grap_byte(my_byte_array, mybp) as u32;
            mybp += 1;
            let cexcept = helpers::grap_byte(my_byte_array, mybp) as u32;
            mybp += 1;

            for k in (0u32..128).step_by(32) {
                bitpacking::fast_unpack(
                    input,
                    tmpinpos as usize,
                    output,
                    (tmpoutpos + k) as usize,
                    bestb as isize,
                );
                tmpinpos += bestb;
            }

            if cexcept > 0 {
                let max_bits = helpers::grap_byte(my_byte_array, mybp) as u32;
                mybp += 1;
                let index = max_bits - bestb;
                let packed_exceptions = &self.data_to_be_packed[index as usize];
                let mut my_index = self.data_pointers[index as usize];

                for _ in 0..cexcept {
                    let pos = helpers::grap_byte(my_byte_array, mybp) as u32;
                    mybp += 1;
                    let except_value = packed_exceptions[my_index as usize];
                    my_index += 1;
                    output[(pos + tmpoutpos) as usize] |= except_value << bestb;
                }
                self.data_pointers[index as usize] = my_index;
            }

            run += 1;
            tmpoutpos += BLOCK_SIZE as u32;
        }

        outpos.set(tmpoutpos as i32);
        inpos.set(inexcept as i32);

        Ok(())
    }

    fn get_best_b_from_data(&mut self, input: &[i32]) -> (i32, i32, i32) {
        self.freqs.clone_from_slice(&ZERO_DATA_POINTERS);

        for v in input {
            self.freqs[helpers::leading_bit_position(*v as u32) as usize] += 1;
        }

        let mut bestb = 32;
        while self.freqs[bestb as usize] == 0 {
            bestb -= 1;
        }

        let maxb = bestb;
        let mut best_cost = bestb * BLOCK_SIZE;
        let mut cexcept = 0;
        let mut bestc = cexcept;

        for b in (0..bestb).rev() {
            cexcept += self.freqs[(b + 1) as usize];
            if cexcept < 0 {
                break;
            }

            let this_cost =
                cexcept * OVERHEAD_OF_EACH_EXCEPT + cexcept * (maxb - b) + b * BLOCK_SIZE + 8;
            if this_cost < best_cost {
                best_cost = this_cost;
                bestb = b;
                bestc = cexcept;
            }
        }

        (bestb, bestc, maxb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fastpfor = FastPFOR::new(DEFAULT_PAGE_SIZE);
        println!("{:?}", fastpfor.data_to_be_packed);
        // assert_eq!(fastpfor.data_to_be_packed.len(), 32);
        // assert_eq!(
        //     fastpfor.byte_container.len(),
        //     (3 * DEFAULT_PAGE_SIZE / BLOCK_SIZE + DEFAULT_PAGE_SIZE)
        //         .try_into()
        //         .unwrap()
        // );
        // assert_eq!(fastpfor.page_size, DEFAULT_PAGE_SIZE);
        // assert_eq!(fastpfor.data_pointers.len(), 33);
        // assert_eq!(fastpfor.freqs.len(), 33);
    }
}
