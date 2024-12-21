pub trait IntegerCODEC {
    #[allow(dead_code, unused_variables)]
    fn compress(&self, input: &[i32], inpos: &mut usize, output: &mut [i32], outpos: &mut usize) {
        todo!("compress not implemented")
    }
    #[allow(dead_code, unused_variables)]
    fn uncompress(&self, input: &[i32], inpos: &mut usize, output: &mut [i32], outpos: &mut usize) {
        todo!("uncompress not implemented")
    }
}

pub trait ByteIntegerCODEC {
    fn compress(&self, input: &[i32], inpos: &mut usize, output: &mut [u8], outpos: &mut usize);
    fn uncompress(&self, input: &[u8], inpos: &mut usize, output: &mut [i32], outpos: &mut usize);
}

pub trait SkippableByteIntegerCODEC {
    fn headless_compress(
        &self,
        input: &[i32],
        inpos: &mut usize,
        inlength: usize,
        output: &mut [u8],
        outpos: &mut usize,
    );
    fn headless_uncompress(
        &self,
        input: &[u8],
        inpos: &mut usize,
        inlength: usize,
        output: &mut [i32],
        outpos: &mut usize,
        num: usize,
    );
}

pub struct VariableByte {
    buffer: Vec<u8>,
}

impl VariableByte {
    fn extract7bits(i: usize, val: u64) -> u8 {
        ((val >> (7 * i)) & ((1 << 7) - 1)) as u8
    }

    fn extract_7bits_maskless(i: usize, val: u64) -> u8 {
        ((val >> (7 * i)) & 0xFF) as u8
    }
}

impl IntegerCODEC for VariableByte {}
