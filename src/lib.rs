mod bitpacking;
mod bytebuffer;
mod cursor;
mod error;
mod fastpfor;
mod helpers;
mod variable_byte;

pub use error::{FastPForError, FastPForResult};
pub use fastpfor::FastPFOR;

enum Codec {
    FastPFor(fastpfor::FastPFOR),
    VariableByte,
}

#[derive(Debug, PartialEq)]
enum Output {
    Byte(Vec<u8>),
    I32(Vec<i32>),
}

// #[derive(Debug, PartialEq)]
// enum Output<'a> {
//     Byte(&'a mut Vec<u8>),
//     I32(&'a mut Vec<i32>),
// }
