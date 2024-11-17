pub fn grap_byte(input: &[i32], index: u32) -> u8 {
    (input[(index / 4) as usize] >> (24 - (index % 4) * 8)) as u8
}

pub fn floor_by(value: i32, factor: i32) -> i32 {
    value - value % factor
}

pub fn ceil_by(value: i32, factor: i32) -> i32 {
    value + factor - value % factor
}

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
