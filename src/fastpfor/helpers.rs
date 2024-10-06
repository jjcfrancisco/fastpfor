
pub fn grap_byte(input: &[i32], index: u32) -> u8 {
    (input[(index / 4) as usize] >> (24 - (index % 4) * 8)) as u8
}

pub fn floor_by(value: i32, factor: i32) -> i32 {
    value - value % factor
}

pub fn ceil_by(value: i32, factor: i32) -> i32 {
    value + factor - value % factor
}
