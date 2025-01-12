use std::io::Cursor;

mod common;

// fn saul_test() {
//     let codecs = common::get_codecs();
//
//     for mut codec in codecs {
//         for x in 0..50 {
//             let input = vec![2, 3, 4, 5];
//             let mut output: Vec<i32> = vec![0; 90];
//             let mut answer = output.clone();
//             let mut input_offset = Cursor::new(0);
//             let mut output_offset = Cursor::new(0);
//             output_offset.set_position(x as u64);
//             codec
//                 .compress(
//                     &input,
//                     input.len() as i32,
//                     &mut input_offset,
//                     &mut output,
//                     &mut output_offset,
//                 )
//                 .unwrap_or_else(|e| {
//                     panic!("Failed to compress with {}: {:?}", codec.name(), e);
//                 });
//             let len = output_offset.position() as i32 - x;
//             output_offset.set_position(x as u64);
//
//             codec
//                 .uncompress(
//                     &output,
//                     len,
//                     &mut input_offset,
//                     &mut answer,
//                     &mut Cursor::new(0),
//                 )
//                 .unwrap_or_else(|e| {
//                     panic!("Failed to uncompress with {}: {:?}", codec.name(), e);
//                 });
//
//             assert_eq!(input, answer);
//         }
//     }
// }

#[test]
fn test_varying_length() {
    let n = 4096;
    let mut data = vec![0u32; n];
    for k in 0..n {
        data[k] = k as u32;
    }
    let codecs = common::get_codecs();
    for mut codec in codecs {
        for l in 1..128 {
            let mut data_copy = data.clone();
            data_copy.resize(l, 0);
            let mut output_compress = vec![0; data_copy.len() * 4];
            codec
                .compress(
                    &data_copy,
                    data_copy.len() as u32,
                    &mut Cursor::new(0),
                    &mut output_compress,
                    &mut Cursor::new(0),
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to compress with {}: {:?}", codec.name(), e);
                });
            let mut answer = vec![0; l + 1024];
            codec
                .uncompress(
                    &output_compress,
                    output_compress.len() as u32,
                    &mut Cursor::new(0),
                    &mut answer,
                    &mut Cursor::new(0),
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to uncompress with {}: {:?}", codec.name(), e);
                });
            for k in 0..l {
                assert_eq!(answer[k], data[k]);
            }
        }
    }
}

#[test]
fn test_varying_length_two() {
    let n = 128;
    let mut data = vec![0u32; n];
    data[126] = -1i32 as u32;
    let codecs = common::get_codecs();
    for mut codec in codecs {
        for l in 1..128 {
            let mut data_copy = data.clone();
            let mut output_compress = vec![0; data_copy.len() * 4];
            data_copy.resize(l, 0);
            codec
                .compress(
                    &data_copy,
                    data_copy.len() as u32,
                    &mut Cursor::new(0),
                    &mut output_compress,
                    &mut Cursor::new(0),
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to compress with {}: {:?}", codec.name(), e);
                });
            let mut answer = vec![0; data_copy.len() + 1024];
            codec
                .uncompress(
                    &output_compress,
                    128,
                    &mut Cursor::new(0),
                    &mut answer,
                    &mut Cursor::new(0),
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to uncompress with {}: {:?}", codec.name(), e);
                });
            for k in 1..l {
                if answer[k] != data[k] {
                    assert_eq!(answer[k], data[k]);
                }
            }
        }
    }
}

use rand::Rng;

#[test]
fn verity_bitpacking() {
    let n = 32;
    let times = 1000;
    let mut r = rand::thread_rng();
    let mut data = vec![0; n];
    let mut compressed = vec![0; n];
    let mut uncompressed = vec![0; n];

    // data = vec![
    //     2, 2, 0, 2, 2, 2, 3, 1, 0, 1, 2, 3, 0, 0, 3, 2, 3, 1, 0, 1, 0, 1, 3, 1, 0, 2, 1, 3, 1, 0,
    //     2, 1,
    // ];
    //
    // fastpfor::fast_pack(&data, 0, &mut compressed, 0, 2);
    // fastpfor::fast_unpack(&compressed, 0, &mut uncompressed, 0, 2);
    //
    // assert_eq!(uncompressed, data, "Mismatch for bit {}", 1);

    for bit in 0..31 {
        for t in 0..times {
            for k in 0..n {
                data[k] = r.gen_range(0..(1 << bit));
            }
    
            fastpfor::fast_pack(&data, 0, &mut compressed, 0, bit);
            fastpfor::fast_unpack(&compressed, 0, &mut uncompressed, 0, bit);
    
            // assert_eq!(uncompressed, data, "Mismatch for bit {}", bit);
            assert_eq!(uncompressed, data, "Mismatch for bit {} and t {}", bit, t);
        }
    }
}

// Java:
// public void verifyBitPacking() {
//     final int N = 32;
//     final int TIMES = 1000;
//     Random r = new Random();
//     int[] data = new int[N];
//     int[] compressed = new int[N];
//     int[] uncompressed = new int[N];
//     for (int bit = 0; bit < 31; ++bit) {
//         for (int t = 0; t < TIMES; ++t) {
//             for (int k = 0; k < N; ++k) {
//                 data[k] = r.nextInt(1 << bit);
//             }
//             BitPacking.fastpack(data, 0, compressed, 0, bit);
//             BitPacking.fastunpack(compressed, 0, uncompressed, 0, bit);
//             assertArrayEquals(uncompressed, data);
//         }
//     }
// }
