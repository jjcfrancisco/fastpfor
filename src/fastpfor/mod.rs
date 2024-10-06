use crate::{Error, Result};

mod bitpacking;
mod cursor;
mod helpers;

use bytebuffer::ByteBuffer;
use cursor::Cursor;

const DEFAULT_BLOCK_SIZE: i32 = 128;
#[allow(dead_code)]
const OVERHEAD_OF_EACH_EXCEPT: i32 = 8;
#[allow(dead_code)]
const DEFAULT_PAGE_SIZE: i32 = 65536;
const ZERO_DATA_POINTERS: [i32; 32] = [0; 32];

#[allow(dead_code)]
struct FastPFOR {
    data_to_be_packed: Vec<Vec<i32>>,
    byte_container: ByteBuffer,
    page_size: i32,
    data_pointers: Vec<i32>,
    freqs: Vec<i32>,
}

impl FastPFOR {
    pub fn new() -> FastPFOR {
        let mut data_to_be_packed = Vec::new();
        for i in 1..33 {
            data_to_be_packed.push(vec![i; DEFAULT_PAGE_SIZE as usize / 32 * 4]);
        }

        let buffer_size = 3 * DEFAULT_PAGE_SIZE / DEFAULT_BLOCK_SIZE + DEFAULT_PAGE_SIZE;
        let mut byte_container = ByteBuffer::new();
        byte_container.resize(buffer_size as usize);

        FastPFOR {
            data_to_be_packed,
            byte_container,
            page_size: DEFAULT_PAGE_SIZE,
            data_pointers: vec![0; 33],
            freqs: vec![0; 33],
        }
    }

    pub fn compress() {}

    pub fn uncompress(
        &mut self,
        input: &mut [i32],
        inpos: &mut cursor::Cursor,
        inlength: i32,
        output: &mut [i32],
        outpos: &mut cursor::Cursor,
    ) -> Result<()> {
        if inlength == 0 {
            return Err(Error::Uncompress("inlength = 0. No work done.".to_string()));
        }

        let nvalue = input[inpos.value as usize];

        self.data_pointers.clone_from_slice(&ZERO_DATA_POINTERS);

        let finalout = outpos.get() + nvalue as i32;
        while outpos.get() != finalout {
            let thissize = std::cmp::min(self.page_size, finalout - outpos.get()) as isize;
            self.decode_page(input, inpos, output, outpos, thissize);
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn encode_page() {}

    #[allow(dead_code)]
    fn decode_page(
        &mut self,
        input: &mut [i32],
        inpos: &mut Cursor,
        output: &mut [i32],
        outpos: &mut Cursor,
        size: isize,
    ) -> &mut Self {
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
        let run_end = size / DEFAULT_BLOCK_SIZE as isize;

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
            tmpoutpos += DEFAULT_BLOCK_SIZE as u32;
        }

        outpos.set(tmpoutpos as i32);
        inpos.set(inexcept as i32);

        self
    }

    fn get_best_b_from_data() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fastpfor = FastPFOR::new();
        assert_eq!(fastpfor.data_to_be_packed.len(), 32);
        assert_eq!(
            fastpfor.byte_container.len(),
            (3 * DEFAULT_PAGE_SIZE / DEFAULT_BLOCK_SIZE + DEFAULT_PAGE_SIZE)
                .try_into()
                .unwrap()
        );
        assert_eq!(fastpfor.page_size, DEFAULT_PAGE_SIZE);
        assert_eq!(fastpfor.data_pointers.len(), 33);
        assert_eq!(fastpfor.freqs.len(), 33);
    }
}
