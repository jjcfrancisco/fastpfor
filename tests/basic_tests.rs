use fastpfor::Output;
use std::io::Cursor;

mod common;

fn saul_test() {
    let codecs = common::get_codecs();

    for mut codec in codecs {
        for x in 0..50 {
            let a = vec![2, 3, 4, 5];
            let mut b = Output::I32(vec![0; 90]);
            let mut c = Output::I32(vec![0; a.len()]);
            let mut a_offset = Cursor::new(0);
            let mut b_offset = Cursor::new(0);
            b_offset.set_position(x as u64);
            codec
                .compress(&a, &mut a_offset, a.len() as i32, &mut b, &mut b_offset)
                .expect("Failed to compress");
            let comp = {
                match b {
                    Output::I32(output) => output,
                    _ => panic!("Output is not I32"),
                }
            };
            let len = b_offset.position() as i32 - x;
            b_offset.set_position(x as u64);

            let mut c_offset = Cursor::new(0);
            codec
                .uncompress(&comp, &mut b_offset, len, &mut c, &mut c_offset)
                .expect("Failed to uncompress");
            let uncomp = {
                match c {
                    Output::I32(output) => output,
                    _ => panic!("Output is not I32"),
                }
            };

            assert_eq!(a, uncomp);
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
    let codecs = common::get_codecs();
    for mut codec in codecs {
        for l in 1..128 {
            let mut data_copy = data.clone();
            data_copy.resize(l, 0);
            let mut output_compress = Output::I32(vec![0; data_copy.len() * 4]);
            codec
                .compress(
                    &data_copy,
                    &mut Cursor::new(0),
                    data_copy.len() as i32,
                    &mut output_compress,
                    &mut Cursor::new(0),
                )
                .expect("Failed to compress");
            let comp = {
                match output_compress {
                    Output::I32(output) => output,
                    _ => panic!("Output is not I32"),
                }
            };
            let mut answer = Output::I32(vec![0; l + 1024]);
            codec
                .uncompress(
                    &comp,
                    &mut Cursor::new(0),
                    comp.len() as i32,
                    &mut answer,
                    &mut Cursor::new(0),
                )
                .expect("Failed to uncompress");
            let uncomp = {
                match answer {
                    Output::I32(output) => output,
                    _ => panic!("Output is not I32"),
                }
            };
            for k in 0..l {
                assert_eq!(uncomp[k], data[k]);
            }
        }
    }
}

#[test]
fn test_varying_length_two() {
    let n = 128;
    let mut data = vec![0; n];
    data[127] = -1;
    let codecs = common::get_codecs();
    for mut codec in codecs {
        for l in 1..128 {
            let mut data_copy = data.clone();
            let mut output_compress = Output::I32(vec![0; data_copy.len() * 4]);
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
            let comp = {
                match output_compress {
                    Output::I32(ref output) => output,
                    _ => panic!("Output is not I32"),
                }
            };
            let mut answer = Output::I32(vec![0; data_copy.len() + 1024]);
            codec
                .uncompress(
                    &comp,
                    &mut Cursor::new(0),
                    128,
                    &mut answer,
                    &mut Cursor::new(0),
                )
                .expect("Failed to uncompress");
            let uncomp = {
                match answer {
                    Output::I32(output) => output,
                    _ => panic!("Output is not I32"),
                }
            };
            for k in 1..l {
                if uncomp[k] != data[k] {
                    panic!("bug");
                }
            }
        }
    }
}
