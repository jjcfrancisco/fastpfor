use crate::bytebuffer;

const BLOCK_SIZE: i32 = 128;
const OVERHEAD_OF_EACH_EXCEPT: i32 = 8;
const DEFAULT_PAGE_SIZE: i32 = 65536;

#[derive(Debug)]
pub struct FastPFOR128 {
    pub data_to_be_packed: Vec<Vec<i32>>,
    pub bytes_container: bytebuffer::ByteBuffer,
    pub page_size: i32,
    pub data_pointers: Vec<usize>,
    pub freqs: Vec<i32>,
    pub bestbbestcexceptmaxb: [i32; 3],
}

impl FastPFOR128 {
    pub fn new(page_size: i32) -> Self {
        FastPFOR128 {
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

    #[allow(dead_code, unused_variables)]
    pub fn compress(&self, input: &[u32], output: &mut [u32]) -> usize {
        // ...
        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fastpfor128_test() {
        // ...
    }
}
