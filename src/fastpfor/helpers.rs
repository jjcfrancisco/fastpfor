
pub fn grap_byte(input: &[i32], index: u32) -> u8 {
    (input[(index / 4) as usize] >> (24 - (index % 4) * 8)) as u8
}
