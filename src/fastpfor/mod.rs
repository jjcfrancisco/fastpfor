use crate::{Error, Result};

mod cursor;

use bytebuffer::ByteBuffer;

const DEFAULT_BLOCK_SIZE: i32 = 128;
const OVERHEAD_OF_EACH_EXCEPT: i32 = 8;
const DEFAULT_PAGE_SIZE: i32 = 65536;

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
        let zero_data_pointers = [0; 32];

        self.data_pointers.clone_from_slice(&zero_data_pointers);

        let finalout = outpos.get() + nvalue as i32;
        let this_page_size_to_float = (self.page_size as f64).min((finalout - outpos.get()) as f64);
        let this_page_size_to_int = this_page_size_to_float as i32;

        // Decode page

        Ok(())
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
