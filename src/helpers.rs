pub fn greatest_multiple(value: i32, factor: i32) -> i32 {
    value - value % factor
}

pub fn bits(i: i32) -> usize {
    32 - i.leading_zeros() as usize
}

#[expect(dead_code)]
pub fn grap_byte(input: &[i32], index: u32) -> u8 {
    (input[(index / 4) as usize] >> (24 - (index % 4) * 8)) as u8
}

#[expect(dead_code)]
pub fn ceil_by(value: i32, factor: i32) -> i32 {
    value + factor - value % factor
}

#[expect(dead_code)]
pub fn leading_bit_position(x: u32) -> i32 {
    bitlen(x as u64) as i32
}

fn clz(x: u64) -> u64 {
    x.leading_zeros() as u64
}

fn bitlen(x: u64) -> i32 {
    if x == 0 {
        return 0;
    }
    64 - clz(x) as i32
}

pub fn extract7bits(i: i32, val: i64) -> u8 {
    ((val >> (7 * i)) & ((1 << 7) - 1)) as u8
}

pub fn extract_7bits_maskless(i: i32, val: i64) -> u8 {
    (val >> (7 * i)) as u8
}

