use fastpfor::{Codec, Compressor};
use std::io::Cursor;

mod common;

fn saul_test() {
    let codecs = common::get_integer_codecs();

    for codec in codecs {
        // unwrap the codec
        let mut codec = match codec {
            Codec::VariableByte(codec) => codec,
        };
        for x in 0..50 {
            let input = vec![2, 3, 4, 5];
            let mut output: Vec<i32> = vec![0; 90];
            let mut answer = output.clone();
            let mut input_offset = Cursor::new(0);
            let mut output_offset = Cursor::new(0);
            output_offset.set_position(x as u64);
            codec
                .compress(
                    &input,
                    &mut input_offset,
                    input.len() as i32,
                    &mut output,
                    &mut output_offset,
                )
                .expect("Failed to compress");
            let len = output_offset.position() as i32 - x;
            output_offset.set_position(x as u64);

            codec
                .uncompress(
                    &output,
                    &mut input_offset,
                    len,
                    &mut answer,
                    &mut Cursor::new(0),
                )
                .expect("Failed to uncompress");

            assert_eq!(input, answer);
        }
    }
}

#[test]
fn test_varying_length() {
    let n = 4096;
    let mut data = vec![0; n];
    for k in 0..n {
        data[k] = k as i32;
    }
    let codecs = common::get_integer_codecs();
    for codec in codecs {
        let mut codec = match codec {
            Codec::VariableByte(codec) => codec,
        };
        for l in 1..128 {
            let mut data_copy = data.clone();
            data_copy.resize(l, 0);
            let mut output_compress = vec![0; data_copy.len() * 4];
            codec
                .compress(
                    &data_copy,
                    &mut Cursor::new(0),
                    data_copy.len() as i32,
                    &mut output_compress,
                    &mut Cursor::new(0),
                )
                .expect("Failed to compress");
            let mut answer = vec![0; l + 1024];
            codec
                .uncompress(
                    &output_compress,
                    &mut Cursor::new(0),
                    output_compress.len() as i32,
                    &mut answer,
                    &mut Cursor::new(0),
                )
                .expect("Failed to uncompress");
            for k in 0..l {
                assert_eq!(answer[k], data[k]);
            }
        }
    }
}

#[test]
fn test_varying_length_two() {
    let n = 128;
    let mut data = vec![0; n];
    data[127] = -1;
    let codecs = common::get_integer_codecs();
    for codec in codecs {
        let mut codec = match codec {
            Codec::VariableByte(codec) => codec,
        };
        for l in 1..128 {
            let mut data_copy = data.clone();
            let mut output_compress = vec![0; data_copy.len() * 4];
            data_copy.resize(l, 0);
            codec
                .compress(
                    &data_copy,
                    &mut Cursor::new(0),
                    data_copy.len() as i32,
                    &mut output_compress,
                    &mut Cursor::new(0),
                )
                .expect("Failed to compress");
            let mut answer = vec![0; data_copy.len() + 1024];
            codec
                .uncompress(
                    &output_compress,
                    &mut Cursor::new(0),
                    128,
                    &mut answer,
                    &mut Cursor::new(0),
                )
                .expect("Failed to uncompress");
            for k in 1..l {
                if answer[k] != data[k] {
                    panic!("bug");
                }
            }
        }
    }
}
