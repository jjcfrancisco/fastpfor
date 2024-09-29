use crate::{Error, Result};

mod cursor;
mod helpers;
mod bitpacking;

use bytebuffer::ByteBuffer;
use cursor::Cursor;

const DEFAULT_BLOCK_SIZE: i32 = 128;
const OVERHEAD_OF_EACH_EXCEPT: i32 = 8;
const DEFAULT_PAGE_SIZE: i32 = 65536;
const ZERO_DATA_POINTERS: [i32; 32] = [0; 32];

struct FastPFOR {
    data_to_be_packed: Vec<Vec<i32>>,
    byte_container: ByteBuffer,
    page_size: i32,
    data_pointers: Vec<i32>,
    freqs: Vec<i32>,
}

impl FastPFOR {
    fn new() -> FastPFOR {
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

    pub fn uncompress(
        &mut self,
        input: &mut [i32],
        inpos: cursor::Cursor,
        inlength: i32,
        output: &mut [i32],
        outpos: cursor::Cursor,
    ) -> Result<()> {
        if inlength == 0 {
            return Err(Error::Uncompress("inlength = 0. No work done.".to_string()));
        }

        let mut nvalue = input[inpos.value as usize];

        self.data_pointers.clone_from_slice(&ZERO_DATA_POINTERS);

        let finalout = outpos.get() + nvalue as i32;
        let this_page_size_to_float = (self.page_size as f64).min((finalout - outpos.get()) as f64);
        let this_page_size_to_int = this_page_size_to_float as i32;

        // Decode page

        Ok(())
    }

    fn decode_page(
        &mut self,
        input: &mut [i32],
        mut inpos: Cursor,
        output: &mut [i32],
        mut outpos: Cursor,
        size: usize,
    ) -> &mut Self {
        let initpos = inpos.get();
        let wheremeta = input[initpos as usize];
        inpos.increment();

        let mut inexcept = initpos + wheremeta;
        let byte_size = input[inexcept as usize] as usize;
        inexcept = inexcept + 1;
        let mybytearray = &input[inexcept as usize..];

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
                    // bitpacking.FastUnpack(in, int(inexcept), this.dataToBePacked[k], int(j), int(k))
                    inexcept += k as i32;
                }
            }
        }

        self.data_pointers.clone_from_slice(&ZERO_DATA_POINTERS);
        let mut tmpoutpos = outpos.get() as u32;
        let mut tmpinpos = inpos.get() as u32;

        let mut run = 0;
        let run_end = size / DEFAULT_BLOCK_SIZE as usize;

        while run < run_end {
            let bestb = helpers::grap_byte(mybytearray, mybp) as u32;
            mybp += 1;
            let cexcept = helpers::grap_byte(mybytearray, mybp) as u32;
            mybp += 1;

            for _ in (0u32..128).step_by(32) {
                // bitpacking.FastUnpack(in, int(tmpinpos), out, int(tmpoutpos+k), int(bestb))
                tmpinpos += bestb;
            }

            if cexcept > 0 {
                let max_bits = helpers::grap_byte(mybytearray, mybp) as u32;
                mybp += 1;
                let index = max_bits - bestb;
                let packed_exceptions = &self.data_to_be_packed[index as usize];
                let mut my_index = self.data_pointers[index as usize];

                for _ in 0..cexcept {
                    let pos = helpers::grap_byte(mybytearray, mybp) as u32;
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
