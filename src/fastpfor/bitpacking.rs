use crate::fastpfor::cursor::Cursor;

pub fn fast_unpack(
    input: &mut [i32],
    mut inpos: i32,
    output: &mut [i32],
    mut outpos: i32,
    bit: i32,
) {
    match bit {
        0 => fast_unpack0(input, inpos, output, outpos),
        1 => fast_unpack1(input, inpos, output, outpos),
        2 => fast_unpack2(input, inpos, output, outpos),
        3 => fast_unpack3(input, inpos, output, outpos),
        4 => fast_unpack4(input, inpos, output, outpos),
        5 => fast_unpack5(input, inpos, output, outpos),
        6 => fast_unpack6(input, inpos, output, outpos),
        7 => fast_unpack7(input, inpos, output, outpos),
        8 => fast_unpack8(input, inpos, output, outpos),
        9 => fast_unpack9(input, inpos, output, outpos),
        10 => fast_unpack10(input, inpos, output, outpos),
        11 => fast_unpack11(input, inpos, output, outpos),
        12 => fast_unpack12(input, inpos, output, outpos),
        13 => fast_unpack13(input, inpos, output, outpos),
        14 => fast_unpack14(input, inpos, output, outpos),
        15 => fast_unpack15(input, inpos, output, outpos),
        16 => fast_unpack16(input, inpos, output, outpos),
        17 => fast_unpack17(input, inpos, output, outpos),
        18 => fast_unpack18(input, inpos, output, outpos),
        19 => fast_unpack19(input, inpos, output, outpos),
        20 => fast_unpack20(input, inpos, output, outpos),
        21 => fast_unpack21(input, inpos, output, outpos),
        22 => fast_unpack22(input, inpos, output, outpos),
        23 => fast_unpack23(input, inpos, output, outpos),
        24 => fast_unpack24(input, inpos, output, outpos),
        25 => fast_unpack25(input, inpos, output, outpos),
        26 => fast_unpack26(input, inpos, output, outpos),
        27 => fast_unpack27(input, inpos, output, outpos),
        28 => fast_unpack28(input, inpos, output, outpos),
        29 => fast_unpack29(input, inpos, output, outpos),
        30 => fast_unpack30(input, inpos, output, outpos),
        31 => fast_unpack31(input, inpos, output, outpos),
        32 => fast_unpack32(input, inpos, output, outpos),
        _ => panic!("Unsupported bit width"),
    }
}

fn fast_unpack0(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    for i in outpos..outpos + 32 {
        output[i as usize] = 0;
    }
}

fn fast_unpack1(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 1;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 1) & 1;
    output[2 + outpos as usize] = (input[0 + inpos as usize] >> 2) & 1;
    output[3 + outpos as usize] = (input[0 + inpos as usize] >> 3) & 1;
    output[4 + outpos as usize] = (input[0 + inpos as usize] >> 4) & 1;
    output[5 + outpos as usize] = (input[0 + inpos as usize] >> 5) & 1;
    output[6 + outpos as usize] = (input[0 + inpos as usize] >> 6) & 1;
    output[7 + outpos as usize] = (input[0 + inpos as usize] >> 7) & 1;
    output[8 + outpos as usize] = (input[0 + inpos as usize] >> 8) & 1;
    output[9 + outpos as usize] = (input[0 + inpos as usize] >> 9) & 1;
    output[10 + outpos as usize] = (input[0 + inpos as usize] >> 10) & 1;
    output[11 + outpos as usize] = (input[0 + inpos as usize] >> 11) & 1;
    output[12 + outpos as usize] = (input[0 + inpos as usize] >> 12) & 1;
    output[13 + outpos as usize] = (input[0 + inpos as usize] >> 13) & 1;
    output[14 + outpos as usize] = (input[0 + inpos as usize] >> 14) & 1;
    output[15 + outpos as usize] = (input[0 + inpos as usize] >> 15) & 1;
    output[16 + outpos as usize] = (input[0 + inpos as usize] >> 16) & 1;
    output[17 + outpos as usize] = (input[0 + inpos as usize] >> 17) & 1;
    output[18 + outpos as usize] = (input[0 + inpos as usize] >> 18) & 1;
    output[19 + outpos as usize] = (input[0 + inpos as usize] >> 19) & 1;
    output[20 + outpos as usize] = (input[0 + inpos as usize] >> 20) & 1;
    output[21 + outpos as usize] = (input[0 + inpos as usize] >> 21) & 1;
    output[22 + outpos as usize] = (input[0 + inpos as usize] >> 22) & 1;
    output[23 + outpos as usize] = (input[0 + inpos as usize] >> 23) & 1;
    output[24 + outpos as usize] = (input[0 + inpos as usize] >> 24) & 1;
    output[25 + outpos as usize] = (input[0 + inpos as usize] >> 25) & 1;
    output[26 + outpos as usize] = (input[0 + inpos as usize] >> 26) & 1;
    output[27 + outpos as usize] = (input[0 + inpos as usize] >> 27) & 1;
    output[28 + outpos as usize] = (input[0 + inpos as usize] >> 28) & 1;
    output[29 + outpos as usize] = (input[0 + inpos as usize] >> 29) & 1;
    output[30 + outpos as usize] = (input[0 + inpos as usize] >> 30) & 1;
    output[31 + outpos as usize] = input[0 + inpos as usize] >> 31;
}

fn fast_unpack2(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 3;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 2) & 3;
    output[2 + outpos as usize] = (input[0 + inpos as usize] >> 4) & 3;
    output[3 + outpos as usize] = (input[0 + inpos as usize] >> 6) & 3;
    output[4 + outpos as usize] = (input[0 + inpos as usize] >> 8) & 3;
    output[5 + outpos as usize] = (input[0 + inpos as usize] >> 10) & 3;
    output[6 + outpos as usize] = (input[0 + inpos as usize] >> 12) & 3;
    output[7 + outpos as usize] = (input[0 + inpos as usize] >> 14) & 3;
    output[8 + outpos as usize] = (input[0 + inpos as usize] >> 16) & 3;
    output[9 + outpos as usize] = (input[0 + inpos as usize] >> 18) & 3;
    output[10 + outpos as usize] = (input[0 + inpos as usize] >> 20) & 3;
    output[11 + outpos as usize] = (input[0 + inpos as usize] >> 22) & 3;
    output[12 + outpos as usize] = (input[0 + inpos as usize] >> 24) & 3;
    output[13 + outpos as usize] = (input[0 + inpos as usize] >> 26) & 3;
    output[14 + outpos as usize] = (input[0 + inpos as usize] >> 28) & 3;
    output[15 + outpos as usize] = input[0 + inpos as usize] >> 30;
    output[16 + outpos as usize] = (input[1 + inpos as usize] >> 0) & 3;
    output[17 + outpos as usize] = (input[1 + inpos as usize] >> 2) & 3;
    output[18 + outpos as usize] = (input[1 + inpos as usize] >> 4) & 3;
    output[19 + outpos as usize] = (input[1 + inpos as usize] >> 6) & 3;
    output[20 + outpos as usize] = (input[1 + inpos as usize] >> 8) & 3;
    output[21 + outpos as usize] = (input[1 + inpos as usize] >> 10) & 3;
    output[22 + outpos as usize] = (input[1 + inpos as usize] >> 12) & 3;
    output[23 + outpos as usize] = (input[1 + inpos as usize] >> 14) & 3;
    output[24 + outpos as usize] = (input[1 + inpos as usize] >> 16) & 3;
    output[25 + outpos as usize] = (input[1 + inpos as usize] >> 18) & 3;
    output[26 + outpos as usize] = (input[1 + inpos as usize] >> 20) & 3;
    output[27 + outpos as usize] = (input[1 + inpos as usize] >> 22) & 3;
    output[28 + outpos as usize] = (input[1 + inpos as usize] >> 24) & 3;
    output[29 + outpos as usize] = (input[1 + inpos as usize] >> 26) & 3;
    output[30 + outpos as usize] = (input[1 + inpos as usize] >> 28) & 3;
    output[31 + outpos as usize] = input[1 + inpos as usize] >> 30;
}

fn fast_unpack3(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 7;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 3) & 7;
    output[2 + outpos as usize] = (input[0 + inpos as usize] >> 6) & 7;
    output[3 + outpos as usize] = (input[0 + inpos as usize] >> 9) & 7;
    output[4 + outpos as usize] = (input[0 + inpos as usize] >> 12) & 7;
    output[5 + outpos as usize] = (input[0 + inpos as usize] >> 15) & 7;
    output[6 + outpos as usize] = (input[0 + inpos as usize] >> 18) & 7;
    output[7 + outpos as usize] = (input[0 + inpos as usize] >> 21) & 7;
    output[8 + outpos as usize] = (input[0 + inpos as usize] >> 24) & 7;
    output[9 + outpos as usize] = (input[0 + inpos as usize] >> 27) & 7;
    output[10 + outpos as usize] =
        input[0 + inpos as usize] >> 30 | (input[1 + inpos as usize] & 1) << (3 - 1);
    output[11 + outpos as usize] = (input[1 + inpos as usize] >> 1) & 7;
    output[12 + outpos as usize] = (input[1 + inpos as usize] >> 4) & 7;
    output[13 + outpos as usize] = (input[1 + inpos as usize] >> 7) & 7;
    output[14 + outpos as usize] = (input[1 + inpos as usize] >> 10) & 7;
    output[15 + outpos as usize] = (input[1 + inpos as usize] >> 13) & 7;
    output[16 + outpos as usize] = (input[1 + inpos as usize] >> 16) & 7;
    output[17 + outpos as usize] = (input[1 + inpos as usize] >> 19) & 7;
    output[18 + outpos as usize] = (input[1 + inpos as usize] >> 22) & 7;
    output[19 + outpos as usize] = (input[1 + inpos as usize] >> 25) & 7;
    output[20 + outpos as usize] = (input[1 + inpos as usize] >> 28) & 7;
    output[21 + outpos as usize] =
        input[1 + inpos as usize] >> 31 | (input[2 + inpos as usize] & 3) << (3 - 2);
    output[22 + outpos as usize] = (input[2 + inpos as usize] >> 2) & 7;
    output[23 + outpos as usize] = (input[2 + inpos as usize] >> 5) & 7;
    output[24 + outpos as usize] = (input[2 + inpos as usize] >> 8) & 7;
    output[25 + outpos as usize] = (input[2 + inpos as usize] >> 11) & 7;
    output[26 + outpos as usize] = (input[2 + inpos as usize] >> 14) & 7;
    output[27 + outpos as usize] = (input[2 + inpos as usize] >> 17) & 7;
    output[28 + outpos as usize] = (input[2 + inpos as usize] >> 20) & 7;
    output[29 + outpos as usize] = (input[2 + inpos as usize] >> 23) & 7;
    output[30 + outpos as usize] = (input[2 + inpos as usize] >> 26) & 7;
    output[31 + outpos as usize] = input[2 + inpos as usize] >> 29;
}

fn fast_unpack4(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 15;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 4) & 15;
    output[2 + outpos as usize] = (input[0 + inpos as usize] >> 8) & 15;
    output[3 + outpos as usize] = (input[0 + inpos as usize] >> 12) & 15;
    output[4 + outpos as usize] = (input[0 + inpos as usize] >> 16) & 15;
    output[5 + outpos as usize] = (input[0 + inpos as usize] >> 20) & 15;
    output[6 + outpos as usize] = (input[0 + inpos as usize] >> 24) & 15;
    output[7 + outpos as usize] = input[0 + inpos as usize] >> 28;
    output[8 + outpos as usize] = (input[1 + inpos as usize] >> 0) & 15;
    output[9 + outpos as usize] = (input[1 + inpos as usize] >> 4) & 15;
    output[10 + outpos as usize] = (input[1 + inpos as usize] >> 8) & 15;
    output[11 + outpos as usize] = (input[1 + inpos as usize] >> 12) & 15;
    output[12 + outpos as usize] = (input[1 + inpos as usize] >> 16) & 15;
    output[13 + outpos as usize] = (input[1 + inpos as usize] >> 20) & 15;
    output[14 + outpos as usize] = (input[1 + inpos as usize] >> 24) & 15;
    output[15 + outpos as usize] = input[1 + inpos as usize] >> 28;
    output[16 + outpos as usize] = (input[2 + inpos as usize] >> 0) & 15;
    output[17 + outpos as usize] = (input[2 + inpos as usize] >> 4) & 15;
    output[18 + outpos as usize] = (input[2 + inpos as usize] >> 8) & 15;
    output[19 + outpos as usize] = (input[2 + inpos as usize] >> 12) & 15;
    output[20 + outpos as usize] = (input[2 + inpos as usize] >> 16) & 15;
    output[21 + outpos as usize] = (input[2 + inpos as usize] >> 20) & 15;
    output[22 + outpos as usize] = (input[2 + inpos as usize] >> 24) & 15;
    output[23 + outpos as usize] = input[2 + inpos as usize] >> 28;
    output[24 + outpos as usize] = (input[3 + inpos as usize] >> 0) & 15;
    output[25 + outpos as usize] = (input[3 + inpos as usize] >> 4) & 15;
    output[26 + outpos as usize] = (input[3 + inpos as usize] >> 8) & 15;
    output[27 + outpos as usize] = (input[3 + inpos as usize] >> 12) & 15;
    output[28 + outpos as usize] = (input[3 + inpos as usize] >> 16) & 15;
    output[29 + outpos as usize] = (input[3 + inpos as usize] >> 20) & 15;
    output[30 + outpos as usize] = (input[3 + inpos as usize] >> 24) & 15;
    output[31 + outpos as usize] = input[3 + inpos as usize] >> 28;
}

fn fast_unpack5(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 31;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 5) & 31;
    output[2 + outpos as usize] = (input[0 + inpos as usize] >> 10) & 31;
    output[3 + outpos as usize] = (input[0 + inpos as usize] >> 15) & 31;
    output[4 + outpos as usize] = (input[0 + inpos as usize] >> 20) & 31;
    output[5 + outpos as usize] = (input[0 + inpos as usize] >> 25) & 31;
    output[6 + outpos as usize] =
        input[0 + inpos as usize] >> 30 | (input[1 + inpos as usize] & 7) << (5 - 3);
    output[7 + outpos as usize] = (input[1 + inpos as usize] >> 3) & 31;
    output[8 + outpos as usize] = (input[1 + inpos as usize] >> 8) & 31;
    output[9 + outpos as usize] = (input[1 + inpos as usize] >> 13) & 31;
    output[10 + outpos as usize] = (input[1 + inpos as usize] >> 18) & 31;
    output[11 + outpos as usize] = (input[1 + inpos as usize] >> 23) & 31;
    output[12 + outpos as usize] =
        input[1 + inpos as usize] >> 28 | (input[2 + inpos as usize] & 1) << (5 - 1);
    output[13 + outpos as usize] = (input[2 + inpos as usize] >> 1) & 31;
    output[14 + outpos as usize] = (input[2 + inpos as usize] >> 6) & 31;
    output[15 + outpos as usize] = (input[2 + inpos as usize] >> 11) & 31;
    output[16 + outpos as usize] = (input[2 + inpos as usize] >> 16) & 31;
    output[17 + outpos as usize] = (input[2 + inpos as usize] >> 21) & 31;
    output[18 + outpos as usize] = (input[2 + inpos as usize] >> 26) & 31;
    output[19 + outpos as usize] =
        input[2 + inpos as usize] >> 31 | (input[3 + inpos as usize] & 15) << (5 - 4);
    output[20 + outpos as usize] = (input[3 + inpos as usize] >> 4) & 31;
    output[21 + outpos as usize] = (input[3 + inpos as usize] >> 9) & 31;
    output[22 + outpos as usize] = (input[3 + inpos as usize] >> 14) & 31;
    output[23 + outpos as usize] = (input[3 + inpos as usize] >> 19) & 31;
    output[24 + outpos as usize] = (input[3 + inpos as usize] >> 24) & 31;
    output[25 + outpos as usize] =
        input[3 + inpos as usize] >> 29 | (input[4 + inpos as usize] & 3) << (5 - 2);
    output[26 + outpos as usize] = (input[4 + inpos as usize] >> 2) & 31;
    output[27 + outpos as usize] = (input[4 + inpos as usize] >> 7) & 31;
    output[28 + outpos as usize] = (input[4 + inpos as usize] >> 12) & 31;
    output[29 + outpos as usize] = (input[4 + inpos as usize] >> 17) & 31;
    output[30 + outpos as usize] = (input[4 + inpos as usize] >> 22) & 31;
    output[31 + outpos as usize] = input[4 + inpos as usize] >> 27;
}

fn fast_unpack6(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 63;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 6) & 63;
    output[2 + outpos as usize] = (input[0 + inpos as usize] >> 12) & 63;
    output[3 + outpos as usize] = (input[0 + inpos as usize] >> 18) & 63;
    output[4 + outpos as usize] = (input[0 + inpos as usize] >> 24) & 63;
    output[5 + outpos as usize] =
        input[0 + inpos as usize] >> 30 | (input[1 + inpos as usize] & 15) << (6 - 4);
    output[6 + outpos as usize] = (input[1 + inpos as usize] >> 4) & 63;
    output[7 + outpos as usize] = (input[1 + inpos as usize] >> 10) & 63;
    output[8 + outpos as usize] = (input[1 + inpos as usize] >> 16) & 63;
    output[9 + outpos as usize] = (input[1 + inpos as usize] >> 22) & 63;
    output[10 + outpos as usize] = (input[1 + inpos as usize] >> 28) & 63;
    output[11 + outpos as usize] =
        input[2 + inpos as usize] >> 0 | (input[3 + inpos as usize] & 3) << (6 - 2);
    output[12 + outpos as usize] = (input[3 + inpos as usize] >> 2) & 63;
    output[13 + outpos as usize] = (input[3 + inpos as usize] >> 8) & 63;
    output[14 + outpos as usize] = (input[3 + inpos as usize] >> 14) & 63;
    output[15 + outpos as usize] = (input[3 + inpos as usize] >> 20) & 63;
    output[16 + outpos as usize] = (input[3 + inpos as usize] >> 26) & 63;
    output[17 + outpos as usize] =
        input[3 + inpos as usize] >> 32 | (input[4 + inpos as usize] & 63) << (6 - 6);
    output[18 + outpos as usize] = (input[4 + inpos as usize] >> 6) & 63;
    output[19 + outpos as usize] = (input[4 + inpos as usize] >> 12) & 63;
    output[20 + outpos as usize] = (input[4 + inpos as usize] >> 18) & 63;
    output[21 + outpos as usize] = (input[4 + inpos as usize] >> 24) & 63;
    output[22 + outpos as usize] =
        input[4 + inpos as usize] >> 30 | (input[5 + inpos as usize] & 15) << (6 - 4);
    output[23 + outpos as usize] = (input[5 + inpos as usize] >> 4) & 63;
    output[24 + outpos as usize] = (input[5 + inpos as usize] >> 10) & 63;
    output[25 + outpos as usize] = (input[5 + inpos as usize] >> 16) & 63;
    output[26 + outpos as usize] = (input[5 + inpos as usize] >> 22) & 63;
    output[27 + outpos as usize] = (input[5 + inpos as usize] >> 28) & 63;
    output[28 + outpos as usize] =
        input[6 + inpos as usize] >> 0 | (input[7 + inpos as usize] & 3) << (6 - 2);
    output[29 + outpos as usize] = (input[7 + inpos as usize] >> 2) & 63;
    output[30 + outpos as usize] = (input[7 + inpos as usize] >> 8) & 63;
    output[31 + outpos as usize] = (input[7 + inpos as usize] >> 14) & 63;
}

fn fast_unpack7(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 127;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 7) & 127;
    output[2 + outpos as usize] = (input[0 + inpos as usize] >> 14) & 127;
    output[3 + outpos as usize] = (input[0 + inpos as usize] >> 21) & 127;
    output[4 + outpos as usize] = (input[0 + inpos as usize] >> 28) & 127;
    output[5 + outpos as usize] =
        input[1 + inpos as usize] >> 0 | (input[2 + inpos as usize] & 3) << (7 - 2);
    output[6 + outpos as usize] = (input[2 + inpos as usize] >> 2) & 127;
    output[7 + outpos as usize] = (input[2 + inpos as usize] >> 9) & 127;
    output[8 + outpos as usize] = (input[2 + inpos as usize] >> 16) & 127;
    output[9 + outpos as usize] = (input[2 + inpos as usize] >> 23) & 127;
    output[10 + outpos as usize] = (input[2 + inpos as usize] >> 30) & 127;
    output[11 + outpos as usize] =
        input[3 + inpos as usize] >> 0 | (input[4 + inpos as usize] & 3) << (7 - 2);
    output[12 + outpos as usize] = (input[4 + inpos as usize] >> 2) & 127;
    output[13 + outpos as usize] = (input[4 + inpos as usize] >> 9) & 127;
    output[14 + outpos as usize] = (input[4 + inpos as usize] >> 16) & 127;
    output[15 + outpos as usize] = (input[4 + inpos as usize] >> 23) & 127;
    output[16 + outpos as usize] = (input[4 + inpos as usize] >> 30) & 127;
    output[17 + outpos as usize] =
        input[5 + inpos as usize] >> 0 | (input[6 + inpos as usize] & 3) << (7 - 2);
    output[18 + outpos as usize] = (input[6 + inpos as usize] >> 2) & 127;
    output[19 + outpos as usize] = (input[6 + inpos as usize] >> 9) & 127;
    output[20 + outpos as usize] = (input[6 + inpos as usize] >> 16) & 127;
    output[21 + outpos as usize] = (input[6 + inpos as usize] >> 23) & 127;
    output[22 + outpos as usize] = (input[6 + inpos as usize] >> 30) & 127;
    output[23 + outpos as usize] =
        input[7 + inpos as usize] >> 0 | (input[8 + inpos as usize] & 3) << (7 - 2);    
    output[24 + outpos as usize] = (input[8 + inpos as usize] >> 2) & 127;
    output[25 + outpos as usize] = (input[8 + inpos as usize] >> 9) & 127;
    output[26 + outpos as usize] = (input[8 + inpos as usize] >> 16) & 127;
    output[27 + outpos as usize] = (input[8 + inpos as usize] >> 23) & 127;
    output[28 + outpos as usize] = (input[8 + inpos as usize] >> 30) & 127;
    output[29 + outpos as usize] =
        input[9 + inpos as usize] >> 0 | (input[10 + inpos as usize] & 3) << (7 - 2);
    output[30 + outpos as usize] = (input[10 + inpos as usize] >> 2) & 127;
    output[31 + outpos as usize] = input[10 + inpos as usize] >> 9;
}

fn fast_unpack8(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 255;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 8) & 255;
    output[2 + outpos as usize] = (input[0 + inpos as usize] >> 16) & 255;
    output[3 + outpos as usize] = (input[0 + inpos as usize] >> 24) & 255;
    output[4 + outpos as usize] = input[1 + inpos as usize] & 255;
    output[5 + outpos as usize] = (input[1 + inpos as usize] >> 8) & 255;
    output[6 + outpos as usize] = (input[1 + inpos as usize] >> 16) & 255;
    output[7 + outpos as usize] = (input[1 + inpos as usize] >> 24) & 255;
    output[8 + outpos as usize] = input[2 + inpos as usize] & 255;
    output[9 + outpos as usize] = (input[2 + inpos as usize] >> 8) & 255;
    output[10 + outpos as usize] = (input[2 + inpos as usize] >> 16) & 255;
    output[11 + outpos as usize] = (input[2 + inpos as usize] >> 24) & 255;
    output[12 + outpos as usize] = input[3 + inpos as usize] & 255;
    output[13 + outpos as usize] = (input[3 + inpos as usize] >> 8) & 255;
    output[14 + outpos as usize] = (input[3 + inpos as usize] >> 16) & 255;
    output[15 + outpos as usize] = (input[3 + inpos as usize] >> 24) & 255;
    output[16 + outpos as usize] = input[4 + inpos as usize] & 255;
    output[17 + outpos as usize] = (input[4 + inpos as usize] >> 8) & 255;
    output[18 + outpos as usize] = (input[4 + inpos as usize] >> 16) & 255;
    output[19 + outpos as usize] = (input[4 + inpos as usize] >> 24) & 255;
    output[20 + outpos as usize] = input[5 + inpos as usize] & 255;
    output[21 + outpos as usize] = (input[5 + inpos as usize] >> 8) & 255;
    output[22 + outpos as usize] = (input[5 + inpos as usize] >> 16) & 255;
    output[23 + outpos as usize] = (input[5 + inpos as usize] >> 24) & 255;
    output[24 + outpos as usize] = input[6 + inpos as usize] & 255;
    output[25 + outpos as usize] = (input[6 + inpos as usize] >> 8) & 255;
    output[26 + outpos as usize] = (input[6 + inpos as usize] >> 16) & 255;
    output[27 + outpos as usize] = (input[6 + inpos as usize] >> 24) & 255;
    output[28 + outpos as usize] = input[7 + inpos as usize] & 255;
    output[29 + outpos as usize] = (input[7 + inpos as usize] >> 8) & 255;
    output[30 + outpos as usize] = (input[7 + inpos as usize] >> 16) & 255;
    output[31 + outpos as usize] = input[7 + inpos as usize] >> 24;
}

fn fast_unpack9(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 511;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 9) & 511;
    output[2 + outpos as usize] = (input[0 + inpos as usize] >> 18) & 511;
    output[3 + outpos as usize] = (input[0 + inpos as usize] >> 27) & 511;
    output[4 + outpos as usize] =
        input[1 + inpos as usize] >> 0 | (input[2 + inpos as usize] & 127) << (9 - 7);
    output[5 + outpos as usize] = (input[2 + inpos as usize] >> 7) & 511;
    output[6 + outpos as usize] = (input[2 + inpos as usize] >> 16) & 511;
    output[7 + outpos as usize] = (input[2 + inpos as usize] >> 25) & 511;
    output[8 + outpos as usize] =
        input[3 + inpos as usize] >> 0 | (input[4 + inpos as usize] & 127) << (9 - 7);
    output[9 + outpos as usize] = (input[4 + inpos as usize] >> 7) & 511;
    output[10 + outpos as usize] = (input[4 + inpos as usize] >> 16) & 511;
    output[11 + outpos as usize] = (input[4 + inpos as usize] >> 25) & 511;
    output[12 + outpos as usize] =
        input[5 + inpos as usize] >> 0 | (input[6 + inpos as usize] & 127) << (9 - 7);
    output[13 + outpos as usize] = (input[6 + inpos as usize] >> 7) & 511;
    output[14 + outpos as usize] = (input[6 + inpos as usize] >> 16) & 511;
    output[15 + outpos as usize] = (input[6 + inpos as usize] >> 25) & 511;
    output[16 + outpos as usize] =
        input[7 + inpos as usize] >> 0 | (input[8 + inpos as usize] & 127) << (9 - 7);
    output[17 + outpos as usize] = (input[8 + inpos as usize] >> 7) & 511;
    output[18 + outpos as usize] = (input[8 + inpos as usize] >> 16) & 511;
    output[19 + outpos as usize] = (input[8 + inpos as usize] >> 25) & 511;
    output[20 + outpos as usize] =
        input[9 + inpos as usize] >> 0 | (input[10 + inpos as usize] & 127) << (9 - 7);
    output[21 + outpos as usize] = (input[10 + inpos as usize] >> 7) & 511;
    output[22 + outpos as usize] = (input[10 + inpos as usize] >> 16) & 511;
    output[23 + outpos as usize] = (input[10 + inpos as usize] >> 25) & 511;
    output[24 + outpos as usize] =
        input[11 + inpos as usize] >> 0 | (input[12 + inpos as usize] & 127) << (9 - 7);
    output[25 + outpos as usize] = (input[12 + inpos as usize] >> 7) & 511;
    output[26 + outpos as usize] = (input[12 + inpos as usize] >> 16) & 511;
    output[27 + outpos as usize] = (input[12 + inpos as usize] >> 25) & 511;
    output[28 + outpos as usize] =
        input[13 + inpos as usize] >> 0 | (input[14 + inpos as usize] & 127) << (9 - 7);
    output[29 + outpos as usize] = (input[14 + inpos as usize] >> 7) & 511;
    output[30 + outpos as usize] = (input[14 + inpos as usize] >> 16) & 511;
    output[31 + outpos as usize] = input[14 + inpos as usize] >> 25;
}

fn fast_unpack10(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 1023;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 10) & 1023;
    output[2 + outpos as usize] = (input[0 + inpos as usize] >> 20) & 1023;
    output[3 + outpos as usize] = input[0 + inpos as usize] >> 30 | (input[1 + inpos as usize] & 255) << (10 - 8);
    output[4 + outpos as usize] = (input[1 + inpos as usize] >> 8) & 1023;
    output[5 + outpos as usize] = (input[1 + inpos as usize] >> 18) & 1023;
    output[6 + outpos as usize] = input[1 + inpos as usize] >> 28 | (input[2 + inpos as usize] & 63) << (10 - 6);
    output[7 + outpos as usize] = (input[2 + inpos as usize] >> 6) & 1023;
    output[8 + outpos as usize] = (input[2 + inpos as usize] >> 16) & 1023;
    output[9 + outpos as usize] = input[2 + inpos as usize] >> 26 | (input[3 + inpos as usize] & 15) << (10 - 4);
    output[10 + outpos as usize] = (input[3 + inpos as usize] >> 4) & 1023;
    output[11 + outpos as usize] = (input[3 + inpos as usize] >> 14) & 1023;
    output[12 + outpos as usize] = input[3 + inpos as usize] >> 24 | (input[4 + inpos as usize] & 3) << (10 - 2);
    output[13 + outpos as usize] = (input[4 + inpos as usize] >> 2) & 1023;
    output[14 + outpos as usize] = (input[4 + inpos as usize] >> 12) & 1023;
    output[15 + outpos as usize] = input[4 + inpos as usize] >> 22;
    output[16 + outpos as usize] = (input[5 + inpos as usize] >> 0) & 1023;
    output[17 + outpos as usize] = (input[5 + inpos as usize] >> 10) & 1023;
    output[18 + outpos as usize] = (input[5 + inpos as usize] >> 20) & 1023;
    output[19 + outpos as usize] = input[5 + inpos as usize] >> 30 | (input[6 + inpos as usize] & 255) << (10 - 8);
    output[20 + outpos as usize] = (input[6 + inpos as usize] >> 8) & 1023;
    output[21 + outpos as usize] = (input[6 + inpos as usize] >> 18) & 1023;
    output[22 + outpos as usize] = input[6 + inpos as usize] >> 28 | (input[7 + inpos as usize] & 63) << (10 - 6);
    output[23 + outpos as usize] = (input[7 + inpos as usize] >> 6) & 1023;
    output[24 + outpos as usize] = (input[7 + inpos as usize] >> 16) & 1023;
    output[25 + outpos as usize] = input[7 + inpos as usize] >> 26 | (input[8 + inpos as usize] & 15) << (10 - 4);
    output[26 + outpos as usize] = (input[8 + inpos as usize] >> 4) & 1023;
    output[27 + outpos as usize] = (input[8 + inpos as usize] >> 14) & 1023;
    output[28 + outpos as usize] = input[8 + inpos as usize] >> 24 | (input[9 + inpos as usize] & 3) << (10 - 2);
    output[29 + outpos as usize] = (input[9 + inpos as usize] >> 2) & 1023;
    output[30 + outpos as usize] = (input[9 + inpos as usize] >> 12) & 1023;
    output[31 + outpos as usize] = input[9 + inpos as usize] >> 22;
}

fn fast_unpack11(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 2047;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 11) & 2047;
    output[2 + outpos as usize] = input[0 + inpos as usize] >> 22 | (input[1 + inpos as usize] & 255) << (11 - 8);
    output[3 + outpos as usize] = (input[1 + inpos as usize] >> 8) & 2047;
    output[4 + outpos as usize] = (input[1 + inpos as usize] >> 19) & 2047;
    output[5 + outpos as usize] = input[1 + inpos as usize] >> 30 | (input[2 + inpos as usize] & 127) << (11 - 7);
    output[6 + outpos as usize] = (input[2 + inpos as usize] >> 7) & 2047;
    output[7 + outpos as usize] = input[2 + inpos as usize] >> 18 | (input[3 + inpos as usize] & 15) << (11 - 4);
    output[8 + outpos as usize] = (input[3 + inpos as usize] >> 4) & 2047;
    output[9 + outpos as usize] = (input[3 + inpos as usize] >> 15) & 2047;
    output[10 + outpos as usize] = input[3 + inpos as usize] >> 26 | (input[4 + inpos as usize] & 63) << (11 - 6);
    output[11 + outpos as usize] = (input[4 + inpos as usize] >> 6) & 2047;
    output[12 + outpos as usize] = input[4 + inpos as usize] >> 17 | (input[5 + inpos as usize] & 7) << (11 - 3);
    output[13 + outpos as usize] = (input[5 + inpos as usize] >> 3) & 2047;
    output[14 + outpos as usize] = (input[5 + inpos as usize] >> 14) & 2047;
    output[15 + outpos as usize] = input[5 + inpos as usize] >> 25 | (input[6 + inpos as usize] & 31) << (11 - 5);
    output[16 + outpos as usize] = (input[6 + inpos as usize] >> 5) & 2047;
    output[17 + outpos as usize] = input[6 + inpos as usize] >> 16 | (input[7 + inpos as usize] & 3) << (11 - 2);
    output[18 + outpos as usize] = (input[7 + inpos as usize] >> 2) & 2047;
    output[19 + outpos as usize] = (input[7 + inpos as usize] >> 13) & 2047;
    output[20 + outpos as usize] = input[7 + inpos as usize] >> 24 | (input[8 + inpos as usize] & 127) << (11 - 7);
    output[21 + outpos as usize] = (input[8 + inpos as usize] >> 7) & 2047;
    output[22 + outpos as usize] = input[8 + inpos as usize] >> 18 | (input[9 + inpos as usize] & 15) << (11 - 4);
    output[23 + outpos as usize] = (input[9 + inpos as usize] >> 4) & 2047;
    output[24 + outpos as usize] = (input[9 + inpos as usize] >> 15) & 2047;
    output[25 + outpos as usize] = input[9 + inpos as usize] >> 26 | (input[10 + inpos as usize] & 63) << (11 - 6);
    output[26 + outpos as usize] = (input[10 + inpos as usize] >> 6) & 2047;
    output[27 + outpos as usize] = input[10 + inpos as usize] >> 17 | (input[11 + inpos as usize] & 7) << (11 - 3);
    output[28 + outpos as usize] = (input[11 + inpos as usize] >> 3) & 2047;
    output[29 + outpos as usize] = (input[11 + inpos as usize] >> 14) & 2047;
    output[30 + outpos as usize] = input[11 + inpos as usize] >> 25 | (input[12 + inpos as usize] & 31) << (11 - 5);
    output[31 + outpos as usize] = (input[12 + inpos as usize] >> 5) & 2047;
}

fn fast_unpack12(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 4095;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 12) & 4095;
    output[2 + outpos as usize] = input[0 + inpos as usize] >> 24 | (input[1 + inpos as usize] & 15) << (12 - 4);
    output[3 + outpos as usize] = (input[1 + inpos as usize] >> 4) & 4095;
    output[4 + outpos as usize] = (input[1 + inpos as usize] >> 16) & 4095;
    output[5 + outpos as usize] = input[1 + inpos as usize] >> 28 | (input[2 + inpos as usize] & 255) << (12 - 8);
    output[6 + outpos as usize] = (input[2 + inpos as usize] >> 8) & 4095;
    output[7 + outpos as usize] = input[2 + inpos as usize] >> 20 | (input[3 + inpos as usize] & 3) << (12 - 2);
    output[8 + outpos as usize] = (input[3 + inpos as usize] >> 2) & 4095;
    output[9 + outpos as usize] = (input[3 + inpos as usize] >> 14) & 4095;
    output[10 + outpos as usize] = input[3 + inpos as usize] >> 26 | (input[4 + inpos as usize] & 63) << (12 - 6);
    output[11 + outpos as usize] = (input[4 + inpos as usize] >> 6) & 4095;
    output[12 + outpos as usize] = (input[4 + inpos as usize] >> 18) & 4095;
    output[13 + outpos as usize] = input[4 + inpos as usize] >> 30 | (input[5 + inpos as usize] & 1023) << (12 - 10);
    output[14 + outpos as usize] = (input[5 + inpos as usize] >> 10) & 4095;
    output[15 + outpos as usize] = input[5 + inpos as usize] >> 22;
    output[16 + outpos as usize] = (input[6 + inpos as usize] >> 0) & 4095;
    output[17 + outpos as usize] = (input[6 + inpos as usize] >> 12) & 4095;
    output[18 + outpos as usize] = input[6 + inpos as usize] >> 24 | (input[7 + inpos as usize] & 15) << (12 - 4);
    output[19 + outpos as usize] = (input[7 + inpos as usize] >> 4) & 4095;
    output[20 + outpos as usize] = (input[7 + inpos as usize] >> 16) & 4095;
    output[21 + outpos as usize] = input[7 + inpos as usize] >> 28 | (input[8 + inpos as usize] & 255) << (12 - 8);
    output[22 + outpos as usize] = (input[8 + inpos as usize] >> 8) & 4095;
    output[23 + outpos as usize] = input[8 + inpos as usize] >> 20 | (input[9 + inpos as usize] & 3) << (12 - 2);
    output[24 + outpos as usize] = (input[9 + inpos as usize] >> 2) & 4095;
    output[25 + outpos as usize] = (input[9 + inpos as usize] >> 14) & 4095;
    output[26 + outpos as usize] = input[9 + inpos as usize] >> 26 | (input[10 + inpos as usize] & 63) << (12 - 6);
    output[27 + outpos as usize] = (input[10 + inpos as usize] >> 6) & 4095;
    output[28 + outpos as usize] = (input[10 + inpos as usize] >> 18) & 4095;
    output[29 + outpos as usize] = input[10 + inpos as usize] >> 30 | (input[11 + inpos as usize] & 1023) << (12 - 10);
    output[30 + outpos as usize] = (input[11 + inpos as usize] >> 10) & 4095;
    output[31 + outpos as usize] = input[11 + inpos as usize] >> 22;
}

fn fast_unpack13(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 8191;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 13) & 8191;
    output[2 + outpos as usize] = input[0 + inpos as usize] >> 26 | (input[1 + inpos as usize] & 127) << (13 - 7);
    output[3 + outpos as usize] = (input[1 + inpos as usize] >> 7) & 8191;
    output[4 + outpos as usize] = input[1 + inpos as usize] >> 20 | (input[2 + inpos as usize] & 3) << (13 - 2);
    output[5 + outpos as usize] = (input[2 + inpos as usize] >> 2) & 8191;
    output[6 + outpos as usize] = (input[2 + inpos as usize] >> 15) & 8191;
    output[7 + outpos as usize] = input[2 + inpos as usize] >> 28 | (input[3 + inpos as usize] & 255) << (13 - 8);
    output[8 + outpos as usize] = (input[3 + inpos as usize] >> 8) & 8191;
    output[9 + outpos as usize] = input[3 + inpos as usize] >> 21 | (input[4 + inpos as usize] & 7) << (13 - 3);
    output[10 + outpos as usize] = (input[4 + inpos as usize] >> 3) & 8191;
    output[11 + outpos as usize] = (input[4 + inpos as usize] >> 16) & 8191;
    output[12 + outpos as usize] = input[4 + inpos as usize] >> 29 | (input[5 + inpos as usize] & 511) << (13 - 9);
    output[13 + outpos as usize] = (input[5 + inpos as usize] >> 9) & 8191;
    output[14 + outpos as usize] = input[5 + inpos as usize] >> 22;
    output[15 + outpos as usize] = (input[6 + inpos as usize] >> 0) & 8191;
    output[16 + outpos as usize] = (input[6 + inpos as usize] >> 13) & 8191;
    output[17 + outpos as usize] = input[6 + inpos as usize] >> 26 | (input[7 + inpos as usize] & 127) << (13 - 7);
    output[18 + outpos as usize] = (input[7 + inpos as usize] >> 7) & 8191;
    output[19 + outpos as usize] = input[7 + inpos as usize] >> 20 | (input[8 + inpos as usize] & 3) << (13 - 2);
    output[20 + outpos as usize] = (input[8 + inpos as usize] >> 2) & 8191;
    output[21 + outpos as usize] = (input[8 + inpos as usize] >> 15) & 8191;
    output[22 + outpos as usize] = input[8 + inpos as usize] >> 28 | (input[9 + inpos as usize] & 255) << (13 - 8);
    output[23 + outpos as usize] = (input[9 + inpos as usize] >> 8) & 8191;
    output[24 + outpos as usize] = input[9 + inpos as usize] >> 21 | (input[10 + inpos as usize] & 7) << (13 - 3);
    output[25 + outpos as usize] = (input[10 + inpos as usize] >> 3) & 8191;
    output[26 + outpos as usize] = (input[10 + inpos as usize] >> 16) & 8191;
    output[27 + outpos as usize] = input[10 + inpos as usize] >> 29 | (input[11 + inpos as usize] & 511) << (13 - 9);
    output[28 + outpos as usize] = (input[11 + inpos as usize] >> 9) & 8191;
    output[29 + outpos as usize] = input[11 + inpos as usize] >> 22;
    output[30 + outpos as usize] = (input[12 + inpos as usize] >> 0) & 8191;
    output[31 + outpos as usize] = (input[12 + inpos as usize] >> 13) & 8191;
}

fn fast_unpack14(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 16383;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 14) & 16383;
    output[2 + outpos as usize] = input[0 + inpos as usize] >> 28 | (input[1 + inpos as usize] & 1023) << (14 - 10);
    output[3 + outpos as usize] = (input[1 + inpos as usize] >> 10) & 16383;
    output[4 + outpos as usize] = input[1 + inpos as usize] >> 24 | (input[2 + inpos as usize] & 63) << (14 - 6);
    output[5 + outpos as usize] = (input[2 + inpos as usize] >> 6) & 16383;
    output[6 + outpos as usize] = input[2 + inpos as usize] >> 20 | (input[3 + inpos as usize] & 3) << (14 - 2);
    output[7 + outpos as usize] = (input[3 + inpos as usize] >> 2) & 16383;
    output[8 + outpos as usize] = (input[3 + inpos as usize] >> 16) & 16383;
    output[9 + outpos as usize] = input[3 + inpos as usize] >> 30 | (input[4 + inpos as usize] & 4095) << (14 - 12);
    output[10 + outpos as usize] = (input[4 + inpos as usize] >> 12) & 16383;
    output[11 + outpos as usize] = input[4 + inpos as usize] >> 26 | (input[5 + inpos as usize] & 255) << (14 - 8);
    output[12 + outpos as usize] = (input[5 + inpos as usize] >> 8) & 16383;
    output[13 + outpos as usize] = input[5 + inpos as usize] >> 22;
    output[14 + outpos as usize] = (input[6 + inpos as usize] >> 0) & 16383;
    output[15 + outpos as usize] = (input[6 + inpos as usize] >> 14) & 16383;
    output[16 + outpos as usize] = input[6 + inpos as usize] >> 28 | (input[7 + inpos as usize] & 1023) << (14 - 10);
    output[17 + outpos as usize] = (input[7 + inpos as usize] >> 10) & 16383;
    output[18 + outpos as usize] = input[7 + inpos as usize] >> 24 | (input[8 + inpos as usize] & 63) << (14 - 6);
    output[19 + outpos as usize] = (input[8 + inpos as usize] >> 6) & 16383;
    output[20 + outpos as usize] = input[8 + inpos as usize] >> 20 | (input[9 + inpos as usize] & 3) << (14 - 2);
    output[21 + outpos as usize] = (input[9 + inpos as usize] >> 2) & 16383;
    output[22 + outpos as usize] = (input[9 + inpos as usize] >> 16) & 16383;
    output[23 + outpos as usize] = input[9 + inpos as usize] >> 30 | (input[10 + inpos as usize] & 4095) << (14 - 12);
    output[24 + outpos as usize] = (input[10 + inpos as usize] >> 12) & 16383;
    output[25 + outpos as usize] = input[10 + inpos as usize] >> 26 | (input[11 + inpos as usize] & 255) << (14 - 8);
    output[26 + outpos as usize] = (input[11 + inpos as usize] >> 8) & 16383;
    output[27 + outpos as usize] = input[11 + inpos as usize] >> 22;
    output[28 + outpos as usize] = (input[12 + inpos as usize] >> 0) & 16383;
    output[29 + outpos as usize] = (input[12 + inpos as usize] >> 14) & 16383;
    output[30 + outpos as usize] = input[12 + inpos as usize] >> 28 | (input[13 + inpos as usize] & 1023) << (14 - 10);
    output[31 + outpos as usize] = (input[13 + inpos as usize] >> 10) & 16383;
}

fn fast_unpack15(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 32767;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 15) & 32767;
    output[2 + outpos as usize] = input[0 + inpos as usize] >> 30 | (input[1 + inpos as usize] & 8191) << (15 - 13);
    output[3 + outpos as usize] = (input[1 + inpos as usize] >> 13) & 32767;
    output[4 + outpos as usize] = input[1 + inpos as usize] >> 28 | (input[2 + inpos as usize] & 4095) << (15 - 12);
    output[5 + outpos as usize] = (input[2 + inpos as usize] >> 12) & 32767;
    output[6 + outpos as usize] = input[2 + inpos as usize] >> 27 | (input[3 + inpos as usize] & 2047) << (15 - 11);
    output[7 + outpos as usize] = (input[3 + inpos as usize] >> 11) & 32767;
    output[8 + outpos as usize] = input[3 + inpos as usize] >> 26 | (input[4 + inpos as usize] & 1023) << (15 - 10);
    output[9 + outpos as usize] = (input[4 + inpos as usize] >> 10) & 32767;
    output[10 + outpos as usize] = input[4 + inpos as usize] >> 25 | (input[5 + inpos as usize] & 511) << (15 - 9);
    output[11 + outpos as usize] = (input[5 + inpos as usize] >> 9) & 32767;
    output[12 + outpos as usize] = input[5 + inpos as usize] >> 24 | (input[6 + inpos as usize] & 255) << (15 - 8);
    output[13 + outpos as usize] = (input[6 + inpos as usize] >> 8) & 32767;
    output[14 + outpos as usize] = input[6 + inpos as usize] >> 23 | (input[7 + inpos as usize] & 127) << (15 - 7);
    output[15 + outpos as usize] = (input[7 + inpos as usize] >> 7) & 32767;
    output[16 + outpos as usize] = input[7 + inpos as usize] >> 22 | (input[8 + inpos as usize] & 63) << (15 - 6);
    output[17 + outpos as usize] = (input[8 + inpos as usize] >> 6) & 32767;
    output[18 + outpos as usize] = input[8 + inpos as usize] >> 21 | (input[9 + inpos as usize] & 31) << (15 - 5);
    output[19 + outpos as usize] = (input[9 + inpos as usize] >> 5) & 32767;
    output[20 + outpos as usize] = input[9 + inpos as usize] >> 20 | (input[10 + inpos as usize] & 15) << (15 - 4);
    output[21 + outpos as usize] = (input[10 + inpos as usize] >> 4) & 32767;
    output[22 + outpos as usize] = input[10 + inpos as usize] >> 19 | (input[11 + inpos as usize] & 7) << (15 - 3);
    output[23 + outpos as usize] = (input[11 + inpos as usize] >> 3) & 32767;
    output[24 + outpos as usize] = input[11 + inpos as usize] >> 18 | (input[12 + inpos as usize] & 3) << (15 - 2);
    output[25 + outpos as usize] = (input[12 + inpos as usize] >> 2) & 32767;
    output[26 + outpos as usize] = (input[12 + inpos as usize] >> 17) & 32767;
    output[27 + outpos as usize] = input[12 + inpos as usize] >> 32 | (input[13 + inpos as usize] & 16383) << (15 - 14);
    output[28 + outpos as usize] = (input[13 + inpos as usize] >> 14) & 32767;
    output[29 + outpos as usize] = input[13 + inpos as usize] >> 29 | (input[14 + inpos as usize] & 8191) << (15 - 13);
    output[30 + outpos as usize] = (input[14 + inpos as usize] >> 13) & 32767;
    output[31 + outpos as usize] = input[14 + inpos as usize] >> 28 | (input[15 + inpos as usize] & 4095) << (15 - 12);
}

fn fast_unpack16(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 65535;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 16) & 65535;
    output[2 + outpos as usize] = input[1 + inpos as usize] & 65535;
    output[3 + outpos as usize] = input[1 + inpos as usize] >> 16;
    output[4 + outpos as usize] = input[2 + inpos as usize] & 65535;
    output[5 + outpos as usize] = input[2 + inpos as usize] >> 16;
    output[6 + outpos as usize] = input[3 + inpos as usize] & 65535;
    output[7 + outpos as usize] = input[3 + inpos as usize] >> 16;
    output[8 + outpos as usize] = input[4 + inpos as usize] & 65535;
    output[9 + outpos as usize] = input[4 + inpos as usize] >> 16;
    output[10 + outpos as usize] = input[5 + inpos as usize] & 65535;
    output[11 + outpos as usize] = input[5 + inpos as usize] >> 16;
    output[12 + outpos as usize] = input[6 + inpos as usize] & 65535;
    output[13 + outpos as usize] = input[6 + inpos as usize] >> 16;
    output[14 + outpos as usize] = input[7 + inpos as usize] & 65535;
    output[15 + outpos as usize] = input[7 + inpos as usize] >> 16;
    output[16 + outpos as usize] = input[8 + inpos as usize] & 65535;
    output[17 + outpos as usize] = input[8 + inpos as usize] >> 16;
    output[18 + outpos as usize] = input[9 + inpos as usize] & 65535;
    output[19 + outpos as usize] = input[9 + inpos as usize] >> 16;
    output[20 + outpos as usize] = input[10 + inpos as usize] & 65535;
    output[21 + outpos as usize] = input[10 + inpos as usize] >> 16;
    output[22 + outpos as usize] = input[11 + inpos as usize] & 65535;
    output[23 + outpos as usize] = input[11 + inpos as usize] >> 16;
    output[24 + outpos as usize] = input[12 + inpos as usize] & 65535;
    output[25 + outpos as usize] = input[12 + inpos as usize] >> 16;
    output[26 + outpos as usize] = input[13 + inpos as usize] & 65535;
    output[27 + outpos as usize] = input[13 + inpos as usize] >> 16;
    output[28 + outpos as usize] = input[14 + inpos as usize] & 65535;
    output[29 + outpos as usize] = input[14 + inpos as usize] >> 16;
    output[30 + outpos as usize] = input[15 + inpos as usize] & 65535;
    output[31 + outpos as usize] = input[15 + inpos as usize] >> 16;
}

fn fast_unpack17(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 131071;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 17) & 131071;
    output[2 + outpos as usize] = input[0 + inpos as usize] >> 34 | (input[1 + inpos as usize] & 8191) << (17 - 13);
    output[3 + outpos as usize] = (input[1 + inpos as usize] >> 13) & 131071;
    output[4 + outpos as usize] = input[1 + inpos as usize] >> 30 | (input[2 + inpos as usize] & 4095) << (17 - 12);
    output[5 + outpos as usize] = (input[2 + inpos as usize] >> 12) & 131071;
    output[6 + outpos as usize] = input[2 + inpos as usize] >> 29 | (input[3 + inpos as usize] & 2047) << (17 - 11);
    output[7 + outpos as usize] = (input[3 + inpos as usize] >> 11) & 131071;
    output[8 + outpos as usize] = input[3 + inpos as usize] >> 28 | (input[4 + inpos as usize] & 1023) << (17 - 10);
    output[9 + outpos as usize] = (input[4 + inpos as usize] >> 10) & 131071;
    output[10 + outpos as usize] = input[4 + inpos as usize] >> 27 | (input[5 + inpos as usize] & 511) << (17 - 9);
    output[11 + outpos as usize] = (input[5 + inpos as usize] >> 9) & 131071;
    output[12 + outpos as usize] = input[5 + inpos as usize] >> 26 | (input[6 + inpos as usize] & 255) << (17 - 8);
    output[13 + outpos as usize] = (input[6 + inpos as usize] >> 8) & 131071;
    output[14 + outpos as usize] = input[6 + inpos as usize] >> 25 | (input[7 + inpos as usize] & 127) << (17 - 7);
    output[15 + outpos as usize] = (input[7 + inpos as usize] >> 7) & 131071;
    output[16 + outpos as usize] = input[7 + inpos as usize] >> 24 | (input[8 + inpos as usize] & 63) << (17 - 6);
    output[17 + outpos as usize] = (input[8 + inpos as usize] >> 6) & 131071;
    output[18 + outpos as usize] = input[8 + inpos as usize] >> 23 | (input[9 + inpos as usize] & 31) << (17 - 5);
    output[19 + outpos as usize] = (input[9 + inpos as usize] >> 5) & 131071;
    output[20 + outpos as usize] = input[9 + inpos as usize] >> 22 | (input[10 + inpos as usize] & 15) << (17 - 4);
    output[21 + outpos as usize] = (input[10 + inpos as usize] >> 4) & 131071;
    output[22 + outpos as usize] = input[10 + inpos as usize] >> 21 | (input[11 + inpos as usize] & 7) << (17 - 3);
    output[23 + outpos as usize] = (input[11 + inpos as usize] >> 3) & 131071;
    output[24 + outpos as usize] = input[11 + inpos as usize] >> 20 | (input[12 + inpos as usize] & 3) << (17 - 2);
    output[25 + outpos as usize] = (input[12 + inpos as usize] >> 2) & 131071;
    output[26 + outpos as usize] = (input[12 + inpos as usize] >> 19) & 131071;
    output[27 + outpos as usize] = input[12 + inpos as usize] >> 36 | (input[13 + inpos as usize] & 32767) << (17 - 15);
    output[28 + outpos as usize] = (input[13 + inpos as usize] >> 15) & 131071;
    output[29 + outpos as usize] = input[13 + inpos as usize] >> 32 | (input[14 + inpos as usize] & 16383) << (17 - 14);
    output[30 + outpos as usize] = (input[14 + inpos as usize] >> 14) & 131071;
    output[31 + outpos as usize] = input[14 + inpos as usize] >> 31 | (input[15 + inpos as usize] & 8191) << (17 - 13);
}

fn fast_unpack18(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 262143;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 18) & 262143;
    output[2 + outpos as usize] = input[0 + inpos as usize] >> 36 | (input[1 + inpos as usize] & 16383) << (18 - 14);
    output[3 + outpos as usize] = (input[1 + inpos as usize] >> 14) & 262143;
    output[4 + outpos as usize] = input[1 + inpos as usize] >> 32 | (input[2 + inpos as usize] & 8191) << (18 - 13);
    output[5 + outpos as usize] = (input[2 + inpos as usize] >> 13) & 262143;
    output[6 + outpos as usize] = input[2 + inpos as usize] >> 31 | (input[3 + inpos as usize] & 4095) << (18 - 12);
    output[7 + outpos as usize] = (input[3 + inpos as usize] >> 12) & 262143;
    output[8 + outpos as usize] = input[3 + inpos as usize] >> 30 | (input[4 + inpos as usize] & 2047) << (18 - 11);
    output[9 + outpos as usize] = (input[4 + inpos as usize] >> 11) & 262143;
    output[10 + outpos as usize] = input[4 + inpos as usize] >> 29 | (input[5 + inpos as usize] & 1023) << (18 - 10);
    output[11 + outpos as usize] = (input[5 + inpos as usize] >> 10) & 262143;
    output[12 + outpos as usize] = input[5 + inpos as usize] >> 28 | (input[6 + inpos as usize] & 511) << (18 - 9);
    output[13 + outpos as usize] = (input[6 + inpos as usize] >> 9) & 262143;
    output[14 + outpos as usize] = input[6 + inpos as usize] >> 27 | (input[7 + inpos as usize] & 255) << (18 - 8);
    output[15 + outpos as usize] = (input[7 + inpos as usize] >> 8) & 262143;
    output[16 + outpos as usize] = input[7 + inpos as usize] >> 26 | (input[8 + inpos as usize] & 127) << (18 - 7);
    output[17 + outpos as usize] = (input[8 + inpos as usize] >> 7) & 262143;
    output[18 + outpos as usize] = input[8 + inpos as usize] >> 25 | (input[9 + inpos as usize] & 63) << (18 - 6);
    output[19 + outpos as usize] = (input[9 + inpos as usize] >> 6) & 262143;
    output[20 + outpos as usize] = input[9 + inpos as usize] >> 24 | (input[10 + inpos as usize] & 31) << (18 - 5);
    output[21 + outpos as usize] = (input[10 + inpos as usize] >> 5) & 262143;
    output[22 + outpos as usize] = input[10 + inpos as usize] >> 23 | (input[11 + inpos as usize] & 15) << (18 - 4);
    output[23 + outpos as usize] = (input[11 + inpos as usize] >> 4) & 262143;
    output[24 + outpos as usize] = input[11 + inpos as usize] >> 22 | (input[12 + inpos as usize] & 7) << (18 - 3);
    output[25 + outpos as usize] = (input[12 + inpos as usize] >> 3) & 262143;
    output[26 + outpos as usize] = input[12 + inpos as usize] >> 21 | (input[13 + inpos as usize] & 3) << (18 - 2);
    output[27 + outpos as usize] = (input[13 + inpos as usize] >> 2) & 262143;
    output[28 + outpos as usize] = (input[13 + inpos as usize] >> 20) & 262143;
    output[29 + outpos as usize] = input[13 + inpos as usize] >> 38 | (input[14 + inpos as usize] & 65535) << (18 - 16);
    output[30 + outpos as usize] = (input[14 + inpos as usize] >> 16) & 262143;
    output[31 + outpos as usize] = input[14 + inpos as usize] >> 34 | (input[15 + inpos as usize] & 32767) << (18 - 15);
}

fn fast_unpack19(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 524287;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 19) & 524287;
    output[2 + outpos as usize] = input[0 + inpos as usize] >> 38 | (input[1 + inpos as usize] & 262143) << (19 - 18);
    output[3 + outpos as usize] = (input[1 + inpos as usize] >> 18) & 524287;
    output[4 + outpos as usize] = input[1 + inpos as usize] >> 37 | (input[2 + inpos as usize] & 131071) << (19 - 17);
    output[5 + outpos as usize] = (input[2 + inpos as usize] >> 17) & 524287;
    output[6 + outpos as usize] = input[2 + inpos as usize] >> 36 | (input[3 + inpos as usize] & 65535) << (19 - 16);
    output[7 + outpos as usize] = (input[3 + inpos as usize] >> 16) & 524287;
    output[8 + outpos as usize] = input[3 + inpos as usize] >> 35 | (input[4 + inpos as usize] & 32767) << (19 - 15);
    output[9 + outpos as usize] = (input[4 + inpos as usize] >> 15) & 524287;
    output[10 + outpos as usize] = input[4 + inpos as usize] >> 34 | (input[5 + inpos as usize] & 16383) << (19 - 14);
    output[11 + outpos as usize] = (input[5 + inpos as usize] >> 14) & 524287;
    output[12 + outpos as usize] = input[5 + inpos as usize] >> 33 | (input[6 + inpos as usize] & 8191) << (19 - 13);
    output[13 + outpos as usize] = (input[6 + inpos as usize] >> 13) & 524287;
    output[14 + outpos as usize] = input[6 + inpos as usize] >> 32 | (input[7 + inpos as usize] & 4095) << (19 - 12);
    output[15 + outpos as usize] = (input[7 + inpos as usize] >> 12) & 524287;
    output[16 + outpos as usize] = input[7 + inpos as usize] >> 31 | (input[8 + inpos as usize] & 2047) << (19 - 11);
    output[17 + outpos as usize] = (input[8 + inpos as usize] >> 11) & 524287;
    output[18 + outpos as usize] = input[8 + inpos as usize] >> 30 | (input[9 + inpos as usize] & 1023) << (19 - 10);
    output[19 + outpos as usize] = (input[9 + inpos as usize] >> 10) & 524287;
    output[20 + outpos as usize] = input[9 + inpos as usize] >> 29 | (input[10 + inpos as usize] & 511) << (19 - 9);
    output[21 + outpos as usize] = (input[10 + inpos as usize] >> 9) & 524287;
    output[22 + outpos as usize] = input[10 + inpos as usize] >> 28 | (input[11 + inpos as usize] & 255) << (19 - 8);
    output[23 + outpos as usize] = (input[11 + inpos as usize] >> 8) & 524287;
    output[24 + outpos as usize] = input[11 + inpos as usize] >> 27 | (input[12 + inpos as usize] & 127) << (19 - 7);
    output[25 + outpos as usize] = (input[12 + inpos as usize] >> 7) & 524287;
    output[26 + outpos as usize] = input[12 + inpos as usize] >> 26 | (input[13 + inpos as usize] & 63) << (19 - 6);
    output[27 + outpos as usize] = (input[13 + inpos as usize] >> 6) & 524287;
    output[28 + outpos as usize] = input[13 + inpos as usize] >> 25 | (input[14 + inpos as usize] & 31) << (19 - 5);
    output[29 + outpos as usize] = (input[14 + inpos as usize] >> 5) & 524287;
    output[30 + outpos as usize] = input[14 + inpos as usize] >> 24 | (input[15 + inpos as usize] & 15) << (19 - 4);
    output[31 + outpos as usize] = (input[15 + inpos as usize] >> 4) & 524287;
}

fn fast_unpack20(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 1048575;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 20) & 1048575;
    output[2 + outpos as usize] = input[0 + inpos as usize] >> 40 | (input[1 + inpos as usize] & 65535) << (20 - 16);
    output[3 + outpos as usize] = (input[1 + inpos as usize] >> 16) & 1048575;
    output[4 + outpos as usize] = input[1 + inpos as usize] >> 36 | (input[2 + inpos as usize] & 4095) << (20 - 12);
    output[5 + outpos as usize] = (input[2 + inpos as usize] >> 12) & 1048575;
    output[6 + outpos as usize] = input[2 + inpos as usize] >> 32 | (input[3 + inpos as usize] & 255) << (20 - 8);
    output[7 + outpos as usize] = (input[3 + inpos as usize] >> 8) & 1048575;
    output[8 + outpos as usize] = input[3 + inpos as usize] >> 28 | (input[4 + inpos as usize] & 15) << (20 - 4);
    output[9 + outpos as usize] = (input[4 + inpos as usize] >> 4) & 1048575;
    output[10 + outpos as usize] = (input[4 + inpos as usize] >> 24) & 1048575;
    output[11 + outpos as usize] = input[4 + inpos as usize] >> 44 | (input[5 + inpos as usize] & 262143) << (20 - 18);
    output[12 + outpos as usize] = (input[5 + inpos as usize] >> 18) & 1048575;
    output[13 + outpos as usize] = input[5 + inpos as usize] >> 38 | (input[6 + inpos as usize] & 16383) << (20 - 14);
    output[14 + outpos as usize] = (input[6 + inpos as usize] >> 14) & 1048575;
    output[15 + outpos as usize] = input[6 + inpos as usize] >> 34 | (input[7 + inpos as usize] & 1023) << (20 - 10);
    output[16 + outpos as usize] = (input[7 + inpos as usize] >> 10) & 1048575;
    output[17 + outpos as usize] = input[7 + inpos as usize] >> 30 | (input[8 + inpos as usize] & 63) << (20 - 6);
    output[18 + outpos as usize] = (input[8 + inpos as usize] >> 6) & 1048575;
    output[19 + outpos as usize] = input[8 + inpos as usize] >> 26 | (input[9 + inpos as usize] & 3) << (20 - 2);
    output[20 + outpos as usize] = (input[9 + inpos as usize] >> 2) & 1048575;
    output[21 + outpos as usize] = (input[9 + inpos as usize] >> 22) & 1048575;
    output[22 + outpos as usize] = input[9 + inpos as usize] >> 42 | (input[10 + inpos as usize] & 131071) << (20 - 17);
    output[23 + outpos as usize] = (input[10 + inpos as usize] >> 17) & 1048575;
    output[24 + outpos as usize] = input[10 + inpos as usize] >> 37 | (input[11 + inpos as usize] & 8191) << (20 - 13);
    output[25 + outpos as usize] = (input[11 + inpos as usize] >> 13) & 1048575;
    output[26 + outpos as usize] = input[11 + inpos as usize] >> 33 | (input[12 + inpos as usize] & 511) << (20 - 9);
    output[27 + outpos as usize] = (input[12 + inpos as usize] >> 9) & 1048575;
    output[28 + outpos as usize] = input[12 + inpos as usize] >> 29 | (input[13 + inpos as usize] & 31) << (20 - 5);
    output[29 + outpos as usize] = (input[13 + inpos as usize] >> 5) & 1048575;
    output[30 + outpos as usize] = input[13 + inpos as usize] >> 25 | (input[14 + inpos as usize] & 1) << (20 - 1);
    output[31 + outpos as usize] = (input[14 + inpos as usize] >> 1) & 1048575;
}

fn fast_unpack21(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 2097151;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 21) & 2097151;
    output[2 + outpos as usize] = input[0 + inpos as usize] >> 42 | (input[1 + inpos as usize] & 1048575) << (21 - 20);
    output[3 + outpos as usize] = (input[1 + inpos as usize] >> 20) & 2097151;
    output[4 + outpos as usize] = input[1 + inpos as usize] >> 41 | (input[2 + inpos as usize] & 524287) << (21 - 19);
    output[5 + outpos as usize] = (input[2 + inpos as usize] >> 19) & 2097151;
    output[6 + outpos as usize] = input[2 + inpos as usize] >> 40 | (input[3 + inpos as usize] & 262143) << (21 - 18);
    output[7 + outpos as usize] = (input[3 + inpos as usize] >> 18) & 2097151;
    output[8 + outpos as usize] = input[3 + inpos as usize] >> 39 | (input[4 + inpos as usize] & 131071) << (21 - 17);
    output[9 + outpos as usize] = (input[4 + inpos as usize] >> 17) & 2097151;
    output[10 + outpos as usize] = input[4 + inpos as usize] >> 38 | (input[5 + inpos as usize] & 65535) << (21 - 16);
    output[11 + outpos as usize] = (input[5 + inpos as usize] >> 16) & 2097151;
    output[12 + outpos as usize] = input[5 + inpos as usize] >> 37 | (input[6 + inpos as usize] & 32767) << (21 - 15);
    output[13 + outpos as usize] = (input[6 + inpos as usize] >> 15) & 2097151;
    output[14 + outpos as usize] = input[6 + inpos as usize] >> 36 | (input[7 + inpos as usize] & 16383) << (21 - 14);
    output[15 + outpos as usize] = (input[7 + inpos as usize] >> 14) & 2097151;
    output[16 + outpos as usize] = input[7 + inpos as usize] >> 35 | (input[8 + inpos as usize] & 8191) << (21 - 13);
    output[17 + outpos as usize] = (input[8 + inpos as usize] >> 13) & 2097151;
    output[18 + outpos as usize] = input[8 + inpos as usize] >> 34 | (input[9 + inpos as usize] & 4095) << (21 - 12);
    output[19 + outpos as usize] = (input[9 + inpos as usize] >> 12) & 2097151;
    output[20 + outpos as usize] = input[9 + inpos as usize] >> 33 | (input[10 + inpos as usize] & 2047) << (21 - 11);
    output[21 + outpos as usize] = (input[10 + inpos as usize] >> 11) & 2097151;
    output[22 + outpos as usize] = input[10 + inpos as usize] >> 32 | (input[11 + inpos as usize] & 1023) << (21 - 10);
    output[23 + outpos as usize] = (input[11 + inpos as usize] >> 10) & 2097151;
    output[24 + outpos as usize] = input[11 + inpos as usize] >> 31 | (input[12 + inpos as usize] & 511) << (21 - 9);
    output[25 + outpos as usize] = (input[12 + inpos as usize] >> 9) & 2097151;
    output[26 + outpos as usize] = input[12 + inpos as usize] >> 30 | (input[13 + inpos as usize] & 255) << (21 - 8);
    output[27 + outpos as usize] = (input[13 + inpos as usize] >> 8) & 2097151;
    output[28 + outpos as usize] = input[13 + inpos as usize] >> 29 | (input[14 + inpos as usize] & 127) << (21 - 7);
    output[29 + outpos as usize] = (input[14 + inpos as usize] >> 7) & 2097151;
    output[30 + outpos as usize] = input[14 + inpos as usize] >> 28 | (input[15 + inpos as usize] & 63) << (21 - 6);
    output[31 + outpos as usize] = (input[15 + inpos as usize] >> 6) & 2097151;
}

fn fast_unpack22(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
    output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 4194303;
    output[1 + outpos as usize] = (input[0 + inpos as usize] >> 22) & 4194303;
    output[2 + outpos as usize] = input[0 + inpos as usize] >> 44 | (input[1 + inpos as usize] & 1048575) << (22 - 20);
    output[3 + outpos as usize] = (input[1 + inpos as usize] >> 20) & 4194303;
    output[4 + outpos as usize] = input[1 + inpos as usize] >> 42 | (input[2 + inpos as usize] & 262143) << (22 - 18);
    output[5 + outpos as usize] = (input[2 + inpos as usize] >> 18) & 4194303;
    output[6 + outpos as usize] = input[2 + inpos as usize] >> 40 | (input[3 + inpos as usize] & 65535) << (22 - 16);
    output[7 + outpos as usize] = (input[3 + inpos as usize] >> 16) & 4194303;
    output[8 + outpos as usize] = input[3 + inpos as usize] >> 38 | (input[4 + inpos as usize] & 16383) << (22 - 14);
    output[9 + outpos as usize] = (input[4 + inpos as usize] >> 14) & 4194303;
    output[10 + outpos as usize] = input[4 + inpos as usize] >> 36 | (input[5 + inpos as usize] & 4095) << (22 - 12);
    output[11 + outpos as usize] = (input[5 + inpos as usize] >> 12) & 4194303;
    output[12 + outpos as usize] = input[5 + inpos as usize] >> 34 | (input[6 + inpos as usize] & 1023) << (22 - 10);
    output[13 + outpos as usize] = (input[6 + inpos as usize] >> 10) & 4194303;
    output[14 + outpos as usize] = input[6 + inpos as usize] >> 32 | (input[7 + inpos as usize] & 255) << (22 - 8);
    output[15 + outpos as usize] = (input[7 + inpos as usize] >> 8) & 4194303;
    output[16 + outpos as usize] = input[7 + inpos as usize] >> 30 | (input[8 + inpos as usize] & 63) << (22 - 6);
    output[17 + outpos as usize] = (input[8 + inpos as usize] >> 6) & 4194303;
    output[18 + outpos as usize] = input[8 + inpos as usize] >> 28 | (input[9 + inpos as usize] & 15) << (22 - 4);
    output[19 + outpos as usize] = (input[9 + inpos as usize] >> 4) & 4194303;
    output[20 + outpos as usize] = (input[9 + inpos as usize] >> 26) & 4194303;
    output[21 + outpos as usize] = input[9 + inpos as usize] >> 48 | (input[10 + inpos as usize] & 262143) << (22 - 18);
    output[22 + outpos as usize] = (input[10 + inpos as usize] >> 18) & 4194303;
    output[23 + outpos as usize] = input[10 + inpos as usize] >> 40 | (input[11 + inpos as usize] & 65535) << (22 - 16);
    output[24 + outpos as usize] = (input[11 + inpos as usize] >> 16) & 4194303;
    output[25 + outpos as usize] = input[11 + inpos as usize] >> 38 | (input[12 + inpos as usize] & 16383) << (22 - 14);
    output[26 + outpos as usize] = (input[12 + inpos as usize] >> 14) & 4194303;
    output[27 + outpos as usize] = input[12 + inpos as usize] >> 36 | (input[13 + inpos as usize] & 4095) << (22 - 12);
    output[28 + outpos as usize] = (input[13 + inpos as usize] >> 12) & 4194303;
    output[29 + outpos as usize] = input[13 + inpos as usize] >> 34 | (input[14 + inpos as usize] & 1023) << (22 - 10);
    output[30 + outpos as usize] = (input[14 + inpos as usize] >> 10) & 4194303;
    output[31 + outpos as usize] = input[14 + inpos as usize] >> 32 | (input[15 + inpos as usize] & 255) << (22 - 8);
}

//
//func fastunpack4(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 15)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>4) & 15)
//
//	out[2+outpos] = (int32(uint32(in[0+inpos])>>8) & 15)
//
//	out[3+outpos] = (int32(uint32(in[0+inpos])>>12) & 15)
//
//	out[4+outpos] = (int32(uint32(in[0+inpos])>>16) & 15)
//
//	out[5+outpos] = (int32(uint32(in[0+inpos])>>20) & 15)
//
//	out[6+outpos] = (int32(uint32(in[0+inpos])>>24) & 15)
//
//	out[7+outpos] = int32(uint32(in[0+inpos]) >> 28)
//
//	out[8+outpos] = (int32(uint32(in[1+inpos])>>0) & 15)
//
//	out[9+outpos] = (int32(uint32(in[1+inpos])>>4) & 15)
//
//	out[10+outpos] = (int32(uint32(in[1+inpos])>>8) & 15)
//
//	out[11+outpos] = (int32(uint32(in[1+inpos])>>12) & 15)
//
//	out[12+outpos] = (int32(uint32(in[1+inpos])>>16) & 15)
//
//	out[13+outpos] = (int32(uint32(in[1+inpos])>>20) & 15)
//
//	out[14+outpos] = (int32(uint32(in[1+inpos])>>24) & 15)
//
//	out[15+outpos] = int32(uint32(in[1+inpos]) >> 28)
//
//	out[16+outpos] = (int32(uint32(in[2+inpos])>>0) & 15)
//
//	out[17+outpos] = (int32(uint32(in[2+inpos])>>4) & 15)
//
//	out[18+outpos] = (int32(uint32(in[2+inpos])>>8) & 15)
//
//	out[19+outpos] = (int32(uint32(in[2+inpos])>>12) & 15)
//
//	out[20+outpos] = (int32(uint32(in[2+inpos])>>16) & 15)
//
//	out[21+outpos] = (int32(uint32(in[2+inpos])>>20) & 15)
//
//	out[22+outpos] = (int32(uint32(in[2+inpos])>>24) & 15)
//
//	out[23+outpos] = int32(uint32(in[2+inpos]) >> 28)
//
//	out[24+outpos] = (int32(uint32(in[3+inpos])>>0) & 15)
//
//	out[25+outpos] = (int32(uint32(in[3+inpos])>>4) & 15)
//
//	out[26+outpos] = (int32(uint32(in[3+inpos])>>8) & 15)
//
//	out[27+outpos] = (int32(uint32(in[3+inpos])>>12) & 15)
//
//	out[28+outpos] = (int32(uint32(in[3+inpos])>>16) & 15)
//
//	out[29+outpos] = (int32(uint32(in[3+inpos])>>20) & 15)
//
//	out[30+outpos] = (int32(uint32(in[3+inpos])>>24) & 15)
//
//	out[31+outpos] = int32(uint32(in[3+inpos]) >> 28)
//
//}
//
//func fastunpack5(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 31)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>5) & 31)
//
//	out[2+outpos] = (int32(uint32(in[0+inpos])>>10) & 31)
//
//	out[3+outpos] = (int32(uint32(in[0+inpos])>>15) & 31)
//
//	out[4+outpos] = (int32(uint32(in[0+inpos])>>20) & 31)
//
//	out[5+outpos] = (int32(uint32(in[0+inpos])>>25) & 31)
//
//	out[6+outpos] = int32(uint32(in[0+inpos])>>30) |
//		((in[1+inpos] & 7) << (5 - 3))
//
//	out[7+outpos] = (int32(uint32(in[1+inpos])>>3) & 31)
//
//	out[8+outpos] = (int32(uint32(in[1+inpos])>>8) & 31)
//
//	out[9+outpos] = (int32(uint32(in[1+inpos])>>13) & 31)
//
//	out[10+outpos] = (int32(uint32(in[1+inpos])>>18) & 31)
//
//	out[11+outpos] = (int32(uint32(in[1+inpos])>>23) & 31)
//
//	out[12+outpos] = int32(uint32(in[1+inpos])>>28) |
//		((in[2+inpos] & 1) << (5 - 1))
//
//	out[13+outpos] = (int32(uint32(in[2+inpos])>>1) & 31)
//
//	out[14+outpos] = (int32(uint32(in[2+inpos])>>6) & 31)
//
//	out[15+outpos] = (int32(uint32(in[2+inpos])>>11) & 31)
//
//	out[16+outpos] = (int32(uint32(in[2+inpos])>>16) & 31)
//
//	out[17+outpos] = (int32(uint32(in[2+inpos])>>21) & 31)
//
//	out[18+outpos] = (int32(uint32(in[2+inpos])>>26) & 31)
//
//	out[19+outpos] = int32(uint32(in[2+inpos])>>31) |
//		((in[3+inpos] & 15) << (5 - 4))
//
//	out[20+outpos] = (int32(uint32(in[3+inpos])>>4) & 31)
//
//	out[21+outpos] = (int32(uint32(in[3+inpos])>>9) & 31)
//
//	out[22+outpos] = (int32(uint32(in[3+inpos])>>14) & 31)
//
//	out[23+outpos] = (int32(uint32(in[3+inpos])>>19) & 31)
//
//	out[24+outpos] = (int32(uint32(in[3+inpos])>>24) & 31)
//
//	out[25+outpos] = int32(uint32(in[3+inpos])>>29) |
//		((in[4+inpos] & 3) << (5 - 2))
//
//	out[26+outpos] = (int32(uint32(in[4+inpos])>>2) & 31)
//
//	out[27+outpos] = (int32(uint32(in[4+inpos])>>7) & 31)
//
//	out[28+outpos] = (int32(uint32(in[4+inpos])>>12) & 31)
//
//	out[29+outpos] = (int32(uint32(in[4+inpos])>>17) & 31)
//
//	out[30+outpos] = (int32(uint32(in[4+inpos])>>22) & 31)
//
//	out[31+outpos] = int32(uint32(in[4+inpos]) >> 27)
//
//}
//
//func fastunpack6(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 63)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>6) & 63)
//
//	out[2+outpos] = (int32(uint32(in[0+inpos])>>12) & 63)
//
//	out[3+outpos] = (int32(uint32(in[0+inpos])>>18) & 63)
//
//	out[4+outpos] = (int32(uint32(in[0+inpos])>>24) & 63)
//
//	out[5+outpos] = int32(uint32(in[0+inpos])>>30) |
//		((in[1+inpos] & 15) << (6 - 4))
//
//	out[6+outpos] = (int32(uint32(in[1+inpos])>>4) & 63)
//
//	out[7+outpos] = (int32(uint32(in[1+inpos])>>10) & 63)
//
//	out[8+outpos] = (int32(uint32(in[1+inpos])>>16) & 63)
//
//	out[9+outpos] = (int32(uint32(in[1+inpos])>>22) & 63)
//
//	out[10+outpos] = int32(uint32(in[1+inpos])>>28) |
//		((in[2+inpos] & 3) << (6 - 2))
//
//	out[11+outpos] = (int32(uint32(in[2+inpos])>>2) & 63)
//
//	out[12+outpos] = (int32(uint32(in[2+inpos])>>8) & 63)
//
//	out[13+outpos] = (int32(uint32(in[2+inpos])>>14) & 63)
//
//	out[14+outpos] = (int32(uint32(in[2+inpos])>>20) & 63)
//
//	out[15+outpos] = int32(uint32(in[2+inpos]) >> 26)
//
//	out[16+outpos] = (int32(uint32(in[3+inpos])>>0) & 63)
//
//	out[17+outpos] = (int32(uint32(in[3+inpos])>>6) & 63)
//
//	out[18+outpos] = (int32(uint32(in[3+inpos])>>12) & 63)
//
//	out[19+outpos] = (int32(uint32(in[3+inpos])>>18) & 63)
//
//	out[20+outpos] = (int32(uint32(in[3+inpos])>>24) & 63)
//
//	out[21+outpos] = int32(uint32(in[3+inpos])>>30) |
//		((in[4+inpos] & 15) << (6 - 4))
//
//	out[22+outpos] = (int32(uint32(in[4+inpos])>>4) & 63)
//
//	out[23+outpos] = (int32(uint32(in[4+inpos])>>10) & 63)
//
//	out[24+outpos] = (int32(uint32(in[4+inpos])>>16) & 63)
//
//	out[25+outpos] = (int32(uint32(in[4+inpos])>>22) & 63)
//
//	out[26+outpos] = int32(uint32(in[4+inpos])>>28) |
//		((in[5+inpos] & 3) << (6 - 2))
//
//	out[27+outpos] = (int32(uint32(in[5+inpos])>>2) & 63)
//
//	out[28+outpos] = (int32(uint32(in[5+inpos])>>8) & 63)
//
//	out[29+outpos] = (int32(uint32(in[5+inpos])>>14) & 63)
//
//	out[30+outpos] = (int32(uint32(in[5+inpos])>>20) & 63)
//
//	out[31+outpos] = int32(uint32(in[5+inpos]) >> 26)
//
//}
//
//func fastunpack7(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 127)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>7) & 127)
//
//	out[2+outpos] = (int32(uint32(in[0+inpos])>>14) & 127)
//
//	out[3+outpos] = (int32(uint32(in[0+inpos])>>21) & 127)
//
//	out[4+outpos] = int32(uint32(in[0+inpos])>>28) |
//		((in[1+inpos] & 7) << (7 - 3))
//
//	out[5+outpos] = (int32(uint32(in[1+inpos])>>3) & 127)
//
//	out[6+outpos] = (int32(uint32(in[1+inpos])>>10) & 127)
//
//	out[7+outpos] = (int32(uint32(in[1+inpos])>>17) & 127)
//
//	out[8+outpos] = (int32(uint32(in[1+inpos])>>24) & 127)
//
//	out[9+outpos] = int32(uint32(in[1+inpos])>>31) |
//		((in[2+inpos] & 63) << (7 - 6))
//
//	out[10+outpos] = (int32(uint32(in[2+inpos])>>6) & 127)
//
//	out[11+outpos] = (int32(uint32(in[2+inpos])>>13) & 127)
//
//	out[12+outpos] = (int32(uint32(in[2+inpos])>>20) & 127)
//
//	out[13+outpos] = int32(uint32(in[2+inpos])>>27) |
//		((in[3+inpos] & 3) << (7 - 2))
//
//	out[14+outpos] = (int32(uint32(in[3+inpos])>>2) & 127)
//
//	out[15+outpos] = (int32(uint32(in[3+inpos])>>9) & 127)
//
//	out[16+outpos] = (int32(uint32(in[3+inpos])>>16) & 127)
//
//	out[17+outpos] = (int32(uint32(in[3+inpos])>>23) & 127)
//
//	out[18+outpos] = int32(uint32(in[3+inpos])>>30) |
//		((in[4+inpos] & 31) << (7 - 5))
//
//	out[19+outpos] = (int32(uint32(in[4+inpos])>>5) & 127)
//
//	out[20+outpos] = (int32(uint32(in[4+inpos])>>12) & 127)
//
//	out[21+outpos] = (int32(uint32(in[4+inpos])>>19) & 127)
//
//	out[22+outpos] = int32(uint32(in[4+inpos])>>26) |
//		((in[5+inpos] & 1) << (7 - 1))
//
//	out[23+outpos] = (int32(uint32(in[5+inpos])>>1) & 127)
//
//	out[24+outpos] = (int32(uint32(in[5+inpos])>>8) & 127)
//
//	out[25+outpos] = (int32(uint32(in[5+inpos])>>15) & 127)
//
//	out[26+outpos] = (int32(uint32(in[5+inpos])>>22) & 127)
//
//	out[27+outpos] = int32(uint32(in[5+inpos])>>29) |
//		((in[6+inpos] & 15) << (7 - 4))
//
//	out[28+outpos] = (int32(uint32(in[6+inpos])>>4) & 127)
//
//	out[29+outpos] = (int32(uint32(in[6+inpos])>>11) & 127)
//
//	out[30+outpos] = (int32(uint32(in[6+inpos])>>18) & 127)
//
//	out[31+outpos] = int32(uint32(in[6+inpos]) >> 25)
//
//}
//
//func fastunpack8(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 255)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>8) & 255)
//
//	out[2+outpos] = (int32(uint32(in[0+inpos])>>16) & 255)
//
//	out[3+outpos] = int32(uint32(in[0+inpos]) >> 24)
//
//	out[4+outpos] = (int32(uint32(in[1+inpos])>>0) & 255)
//
//	out[5+outpos] = (int32(uint32(in[1+inpos])>>8) & 255)
//
//	out[6+outpos] = (int32(uint32(in[1+inpos])>>16) & 255)
//
//	out[7+outpos] = int32(uint32(in[1+inpos]) >> 24)
//
//	out[8+outpos] = (int32(uint32(in[2+inpos])>>0) & 255)
//
//	out[9+outpos] = (int32(uint32(in[2+inpos])>>8) & 255)
//
//	out[10+outpos] = (int32(uint32(in[2+inpos])>>16) & 255)
//
//	out[11+outpos] = int32(uint32(in[2+inpos]) >> 24)
//
//	out[12+outpos] = (int32(uint32(in[3+inpos])>>0) & 255)
//
//	out[13+outpos] = (int32(uint32(in[3+inpos])>>8) & 255)
//
//	out[14+outpos] = (int32(uint32(in[3+inpos])>>16) & 255)
//
//	out[15+outpos] = int32(uint32(in[3+inpos]) >> 24)
//
//	out[16+outpos] = (int32(uint32(in[4+inpos])>>0) & 255)
//
//	out[17+outpos] = (int32(uint32(in[4+inpos])>>8) & 255)
//
//	out[18+outpos] = (int32(uint32(in[4+inpos])>>16) & 255)
//
//	out[19+outpos] = int32(uint32(in[4+inpos]) >> 24)
//
//	out[20+outpos] = (int32(uint32(in[5+inpos])>>0) & 255)
//
//	out[21+outpos] = (int32(uint32(in[5+inpos])>>8) & 255)
//
//	out[22+outpos] = (int32(uint32(in[5+inpos])>>16) & 255)
//
//	out[23+outpos] = int32(uint32(in[5+inpos]) >> 24)
//
//	out[24+outpos] = (int32(uint32(in[6+inpos])>>0) & 255)
//
//	out[25+outpos] = (int32(uint32(in[6+inpos])>>8) & 255)
//
//	out[26+outpos] = (int32(uint32(in[6+inpos])>>16) & 255)
//
//	out[27+outpos] = int32(uint32(in[6+inpos]) >> 24)
//
//	out[28+outpos] = (int32(uint32(in[7+inpos])>>0) & 255)
//
//	out[29+outpos] = (int32(uint32(in[7+inpos])>>8) & 255)
//
//	out[30+outpos] = (int32(uint32(in[7+inpos])>>16) & 255)
//
//	out[31+outpos] = int32(uint32(in[7+inpos]) >> 24)
//
//}
//
//func fastunpack9(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 511)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>9) & 511)
//
//	out[2+outpos] = (int32(uint32(in[0+inpos])>>18) & 511)
//
//	out[3+outpos] = int32(uint32(in[0+inpos])>>27) |
//		((in[1+inpos] & 15) << (9 - 4))
//
//	out[4+outpos] = (int32(uint32(in[1+inpos])>>4) & 511)
//
//	out[5+outpos] = (int32(uint32(in[1+inpos])>>13) & 511)
//
//	out[6+outpos] = (int32(uint32(in[1+inpos])>>22) & 511)
//
//	out[7+outpos] = int32(uint32(in[1+inpos])>>31) |
//		((in[2+inpos] & 255) << (9 - 8))
//
//	out[8+outpos] = (int32(uint32(in[2+inpos])>>8) & 511)
//
//	out[9+outpos] = (int32(uint32(in[2+inpos])>>17) & 511)
//
//	out[10+outpos] = int32(uint32(in[2+inpos])>>26) |
//		((in[3+inpos] & 7) << (9 - 3))
//
//	out[11+outpos] = (int32(uint32(in[3+inpos])>>3) & 511)
//
//	out[12+outpos] = (int32(uint32(in[3+inpos])>>12) & 511)
//
//	out[13+outpos] = (int32(uint32(in[3+inpos])>>21) & 511)
//
//	out[14+outpos] = int32(uint32(in[3+inpos])>>30) |
//		((in[4+inpos] & 127) << (9 - 7))
//
//	out[15+outpos] = (int32(uint32(in[4+inpos])>>7) & 511)
//
//	out[16+outpos] = (int32(uint32(in[4+inpos])>>16) & 511)
//
//	out[17+outpos] = int32(uint32(in[4+inpos])>>25) |
//		((in[5+inpos] & 3) << (9 - 2))
//
//	out[18+outpos] = (int32(uint32(in[5+inpos])>>2) & 511)
//
//	out[19+outpos] = (int32(uint32(in[5+inpos])>>11) & 511)
//
//	out[20+outpos] = (int32(uint32(in[5+inpos])>>20) & 511)
//
//	out[21+outpos] = int32(uint32(in[5+inpos])>>29) |
//		((in[6+inpos] & 63) << (9 - 6))
//
//	out[22+outpos] = (int32(uint32(in[6+inpos])>>6) & 511)
//
//	out[23+outpos] = (int32(uint32(in[6+inpos])>>15) & 511)
//
//	out[24+outpos] = int32(uint32(in[6+inpos])>>24) |
//		((in[7+inpos] & 1) << (9 - 1))
//
//	out[25+outpos] = (int32(uint32(in[7+inpos])>>1) & 511)
//
//	out[26+outpos] = (int32(uint32(in[7+inpos])>>10) & 511)
//
//	out[27+outpos] = (int32(uint32(in[7+inpos])>>19) & 511)
//
//	out[28+outpos] = int32(uint32(in[7+inpos])>>28) |
//		((in[8+inpos] & 31) << (9 - 5))
//
//	out[29+outpos] = (int32(uint32(in[8+inpos])>>5) & 511)
//
//	out[30+outpos] = (int32(uint32(in[8+inpos])>>14) & 511)
//
//	out[31+outpos] = int32(uint32(in[8+inpos]) >> 23)
//
//}
//
//func fastunpack10(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 1023)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>10) & 1023)
//
//	out[2+outpos] = (int32(uint32(in[0+inpos])>>20) & 1023)
//
//	out[3+outpos] = int32(uint32(in[0+inpos])>>30) |
//		((in[1+inpos] & 255) << (10 - 8))
//
//	out[4+outpos] = (int32(uint32(in[1+inpos])>>8) & 1023)
//
//	out[5+outpos] = (int32(uint32(in[1+inpos])>>18) & 1023)
//
//	out[6+outpos] = int32(uint32(in[1+inpos])>>28) |
//		((in[2+inpos] & 63) << (10 - 6))
//
//	out[7+outpos] = (int32(uint32(in[2+inpos])>>6) & 1023)
//
//	out[8+outpos] = (int32(uint32(in[2+inpos])>>16) & 1023)
//
//	out[9+outpos] = int32(uint32(in[2+inpos])>>26) |
//		((in[3+inpos] & 15) << (10 - 4))
//
//	out[10+outpos] = (int32(uint32(in[3+inpos])>>4) & 1023)
//
//	out[11+outpos] = (int32(uint32(in[3+inpos])>>14) & 1023)
//
//	out[12+outpos] = int32(uint32(in[3+inpos])>>24) |
//		((in[4+inpos] & 3) << (10 - 2))
//
//	out[13+outpos] = (int32(uint32(in[4+inpos])>>2) & 1023)
//
//	out[14+outpos] = (int32(uint32(in[4+inpos])>>12) & 1023)
//
//	out[15+outpos] = int32(uint32(in[4+inpos]) >> 22)
//
//	out[16+outpos] = (int32(uint32(in[5+inpos])>>0) & 1023)
//
//	out[17+outpos] = (int32(uint32(in[5+inpos])>>10) & 1023)
//
//	out[18+outpos] = (int32(uint32(in[5+inpos])>>20) & 1023)
//
//	out[19+outpos] = int32(uint32(in[5+inpos])>>30) |
//		((in[6+inpos] & 255) << (10 - 8))
//
//	out[20+outpos] = (int32(uint32(in[6+inpos])>>8) & 1023)
//
//	out[21+outpos] = (int32(uint32(in[6+inpos])>>18) & 1023)
//
//	out[22+outpos] = int32(uint32(in[6+inpos])>>28) |
//		((in[7+inpos] & 63) << (10 - 6))
//
//	out[23+outpos] = (int32(uint32(in[7+inpos])>>6) & 1023)
//
//	out[24+outpos] = (int32(uint32(in[7+inpos])>>16) & 1023)
//
//	out[25+outpos] = int32(uint32(in[7+inpos])>>26) |
//		((in[8+inpos] & 15) << (10 - 4))
//
//	out[26+outpos] = (int32(uint32(in[8+inpos])>>4) & 1023)
//
//	out[27+outpos] = (int32(uint32(in[8+inpos])>>14) & 1023)
//
//	out[28+outpos] = int32(uint32(in[8+inpos])>>24) |
//		((in[9+inpos] & 3) << (10 - 2))
//
//	out[29+outpos] = (int32(uint32(in[9+inpos])>>2) & 1023)
//
//	out[30+outpos] = (int32(uint32(in[9+inpos])>>12) & 1023)
//
//	out[31+outpos] = int32(uint32(in[9+inpos]) >> 22)
//
//}
//
//func fastunpack11(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 2047)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>11) & 2047)
//
//	out[2+outpos] = int32(uint32(in[0+inpos])>>22) |
//		((in[1+inpos] & 1) << (11 - 1))
//
//	out[3+outpos] = (int32(uint32(in[1+inpos])>>1) & 2047)
//
//	out[4+outpos] = (int32(uint32(in[1+inpos])>>12) & 2047)
//
//	out[5+outpos] = int32(uint32(in[1+inpos])>>23) |
//		((in[2+inpos] & 3) << (11 - 2))
//
//	out[6+outpos] = (int32(uint32(in[2+inpos])>>2) & 2047)
//
//	out[7+outpos] = (int32(uint32(in[2+inpos])>>13) & 2047)
//
//	out[8+outpos] = int32(uint32(in[2+inpos])>>24) |
//		((in[3+inpos] & 7) << (11 - 3))
//
//	out[9+outpos] = (int32(uint32(in[3+inpos])>>3) & 2047)
//
//	out[10+outpos] = (int32(uint32(in[3+inpos])>>14) & 2047)
//
//	out[11+outpos] = int32(uint32(in[3+inpos])>>25) |
//		((in[4+inpos] & 15) << (11 - 4))
//
//	out[12+outpos] = (int32(uint32(in[4+inpos])>>4) & 2047)
//
//	out[13+outpos] = (int32(uint32(in[4+inpos])>>15) & 2047)
//
//	out[14+outpos] = int32(uint32(in[4+inpos])>>26) |
//		((in[5+inpos] & 31) << (11 - 5))
//
//	out[15+outpos] = (int32(uint32(in[5+inpos])>>5) & 2047)
//
//	out[16+outpos] = (int32(uint32(in[5+inpos])>>16) & 2047)
//
//	out[17+outpos] = int32(uint32(in[5+inpos])>>27) |
//		((in[6+inpos] & 63) << (11 - 6))
//
//	out[18+outpos] = (int32(uint32(in[6+inpos])>>6) & 2047)
//
//	out[19+outpos] = (int32(uint32(in[6+inpos])>>17) & 2047)
//
//	out[20+outpos] = int32(uint32(in[6+inpos])>>28) |
//		((in[7+inpos] & 127) << (11 - 7))
//
//	out[21+outpos] = (int32(uint32(in[7+inpos])>>7) & 2047)
//
//	out[22+outpos] = (int32(uint32(in[7+inpos])>>18) & 2047)
//
//	out[23+outpos] = int32(uint32(in[7+inpos])>>29) |
//		((in[8+inpos] & 255) << (11 - 8))
//
//	out[24+outpos] = (int32(uint32(in[8+inpos])>>8) & 2047)
//
//	out[25+outpos] = (int32(uint32(in[8+inpos])>>19) & 2047)
//
//	out[26+outpos] = int32(uint32(in[8+inpos])>>30) |
//		((in[9+inpos] & 511) << (11 - 9))
//
//	out[27+outpos] = (int32(uint32(in[9+inpos])>>9) & 2047)
//
//	out[28+outpos] = (int32(uint32(in[9+inpos])>>20) & 2047)
//
//	out[29+outpos] = int32(uint32(in[9+inpos])>>31) |
//		((in[10+inpos] & 1023) << (11 - 10))
//
//	out[30+outpos] = (int32(uint32(in[10+inpos])>>10) & 2047)
//
//	out[31+outpos] = int32(uint32(in[10+inpos]) >> 21)
//
//}
//
//func fastunpack12(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 4095)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>12) & 4095)
//
//	out[2+outpos] = int32(uint32(in[0+inpos])>>24) |
//		((in[1+inpos] & 15) << (12 - 4))
//
//	out[3+outpos] = (int32(uint32(in[1+inpos])>>4) & 4095)
//
//	out[4+outpos] = (int32(uint32(in[1+inpos])>>16) & 4095)
//
//	out[5+outpos] = int32(uint32(in[1+inpos])>>28) |
//		((in[2+inpos] & 255) << (12 - 8))
//
//	out[6+outpos] = (int32(uint32(in[2+inpos])>>8) & 4095)
//
//	out[7+outpos] = int32(uint32(in[2+inpos]) >> 20)
//
//	out[8+outpos] = (int32(uint32(in[3+inpos])>>0) & 4095)
//
//	out[9+outpos] = (int32(uint32(in[3+inpos])>>12) & 4095)
//
//	out[10+outpos] = int32(uint32(in[3+inpos])>>24) |
//		((in[4+inpos] & 15) << (12 - 4))
//
//	out[11+outpos] = (int32(uint32(in[4+inpos])>>4) & 4095)
//
//	out[12+outpos] = (int32(uint32(in[4+inpos])>>16) & 4095)
//
//	out[13+outpos] = int32(uint32(in[4+inpos])>>28) |
//		((in[5+inpos] & 255) << (12 - 8))
//
//	out[14+outpos] = (int32(uint32(in[5+inpos])>>8) & 4095)
//
//	out[15+outpos] = int32(uint32(in[5+inpos]) >> 20)
//
//	out[16+outpos] = (int32(uint32(in[6+inpos])>>0) & 4095)
//
//	out[17+outpos] = (int32(uint32(in[6+inpos])>>12) & 4095)
//
//	out[18+outpos] = int32(uint32(in[6+inpos])>>24) |
//		((in[7+inpos] & 15) << (12 - 4))
//
//	out[19+outpos] = (int32(uint32(in[7+inpos])>>4) & 4095)
//
//	out[20+outpos] = (int32(uint32(in[7+inpos])>>16) & 4095)
//
//	out[21+outpos] = int32(uint32(in[7+inpos])>>28) |
//		((in[8+inpos] & 255) << (12 - 8))
//
//	out[22+outpos] = (int32(uint32(in[8+inpos])>>8) & 4095)
//
//	out[23+outpos] = int32(uint32(in[8+inpos]) >> 20)
//
//	out[24+outpos] = (int32(uint32(in[9+inpos])>>0) & 4095)
//
//	out[25+outpos] = (int32(uint32(in[9+inpos])>>12) & 4095)
//
//	out[26+outpos] = int32(uint32(in[9+inpos])>>24) |
//		((in[10+inpos] & 15) << (12 - 4))
//
//	out[27+outpos] = (int32(uint32(in[10+inpos])>>4) & 4095)
//
//	out[28+outpos] = (int32(uint32(in[10+inpos])>>16) & 4095)
//
//	out[29+outpos] = int32(uint32(in[10+inpos])>>28) |
//		((in[11+inpos] & 255) << (12 - 8))
//
//	out[30+outpos] = (int32(uint32(in[11+inpos])>>8) & 4095)
//
//	out[31+outpos] = int32(uint32(in[11+inpos]) >> 20)
//
//}
//
//func fastunpack13(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 8191)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>13) & 8191)
//
//	out[2+outpos] = int32(uint32(in[0+inpos])>>26) |
//		((in[1+inpos] & 127) << (13 - 7))
//
//	out[3+outpos] = (int32(uint32(in[1+inpos])>>7) & 8191)
//
//	out[4+outpos] = int32(uint32(in[1+inpos])>>20) |
//		((in[2+inpos] & 1) << (13 - 1))
//
//	out[5+outpos] = (int32(uint32(in[2+inpos])>>1) & 8191)
//
//	out[6+outpos] = (int32(uint32(in[2+inpos])>>14) & 8191)
//
//	out[7+outpos] = int32(uint32(in[2+inpos])>>27) |
//		((in[3+inpos] & 255) << (13 - 8))
//
//	out[8+outpos] = (int32(uint32(in[3+inpos])>>8) & 8191)
//
//	out[9+outpos] = int32(uint32(in[3+inpos])>>21) |
//		((in[4+inpos] & 3) << (13 - 2))
//
//	out[10+outpos] = (int32(uint32(in[4+inpos])>>2) & 8191)
//
//	out[11+outpos] = (int32(uint32(in[4+inpos])>>15) & 8191)
//
//	out[12+outpos] = int32(uint32(in[4+inpos])>>28) |
//		((in[5+inpos] & 511) << (13 - 9))
//
//	out[13+outpos] = (int32(uint32(in[5+inpos])>>9) & 8191)
//
//	out[14+outpos] = int32(uint32(in[5+inpos])>>22) |
//		((in[6+inpos] & 7) << (13 - 3))
//
//	out[15+outpos] = (int32(uint32(in[6+inpos])>>3) & 8191)
//
//	out[16+outpos] = (int32(uint32(in[6+inpos])>>16) & 8191)
//
//	out[17+outpos] = int32(uint32(in[6+inpos])>>29) |
//		((in[7+inpos] & 1023) << (13 - 10))
//
//	out[18+outpos] = (int32(uint32(in[7+inpos])>>10) & 8191)
//
//	out[19+outpos] = int32(uint32(in[7+inpos])>>23) |
//		((in[8+inpos] & 15) << (13 - 4))
//
//	out[20+outpos] = (int32(uint32(in[8+inpos])>>4) & 8191)
//
//	out[21+outpos] = (int32(uint32(in[8+inpos])>>17) & 8191)
//
//	out[22+outpos] = int32(uint32(in[8+inpos])>>30) |
//		((in[9+inpos] & 2047) << (13 - 11))
//
//	out[23+outpos] = (int32(uint32(in[9+inpos])>>11) & 8191)
//
//	out[24+outpos] = int32(uint32(in[9+inpos])>>24) |
//		((in[10+inpos] & 31) << (13 - 5))
//
//	out[25+outpos] = (int32(uint32(in[10+inpos])>>5) & 8191)
//
//	out[26+outpos] = (int32(uint32(in[10+inpos])>>18) & 8191)
//
//	out[27+outpos] = int32(uint32(in[10+inpos])>>31) |
//		((in[11+inpos] & 4095) << (13 - 12))
//
//	out[28+outpos] = (int32(uint32(in[11+inpos])>>12) & 8191)
//
//	out[29+outpos] = int32(uint32(in[11+inpos])>>25) |
//		((in[12+inpos] & 63) << (13 - 6))
//
//	out[30+outpos] = (int32(uint32(in[12+inpos])>>6) & 8191)
//
//	out[31+outpos] = int32(uint32(in[12+inpos]) >> 19)
//
//}
//
//func fastunpack14(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 16383)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>14) & 16383)
//
//	out[2+outpos] = int32(uint32(in[0+inpos])>>28) |
//		((in[1+inpos] & 1023) << (14 - 10))
//
//	out[3+outpos] = (int32(uint32(in[1+inpos])>>10) & 16383)
//
//	out[4+outpos] = int32(uint32(in[1+inpos])>>24) |
//		((in[2+inpos] & 63) << (14 - 6))
//
//	out[5+outpos] = (int32(uint32(in[2+inpos])>>6) & 16383)
//
//	out[6+outpos] = int32(uint32(in[2+inpos])>>20) |
//		((in[3+inpos] & 3) << (14 - 2))
//
//	out[7+outpos] = (int32(uint32(in[3+inpos])>>2) & 16383)
//
//	out[8+outpos] = (int32(uint32(in[3+inpos])>>16) & 16383)
//
//	out[9+outpos] = int32(uint32(in[3+inpos])>>30) |
//		((in[4+inpos] & 4095) << (14 - 12))
//
//	out[10+outpos] = (int32(uint32(in[4+inpos])>>12) & 16383)
//
//	out[11+outpos] = int32(uint32(in[4+inpos])>>26) |
//		((in[5+inpos] & 255) << (14 - 8))
//
//	out[12+outpos] = (int32(uint32(in[5+inpos])>>8) & 16383)
//
//	out[13+outpos] = int32(uint32(in[5+inpos])>>22) |
//		((in[6+inpos] & 15) << (14 - 4))
//
//	out[14+outpos] = (int32(uint32(in[6+inpos])>>4) & 16383)
//
//	out[15+outpos] = int32(uint32(in[6+inpos]) >> 18)
//
//	out[16+outpos] = (int32(uint32(in[7+inpos])>>0) & 16383)
//
//	out[17+outpos] = (int32(uint32(in[7+inpos])>>14) & 16383)
//
//	out[18+outpos] = int32(uint32(in[7+inpos])>>28) |
//		((in[8+inpos] & 1023) << (14 - 10))
//
//	out[19+outpos] = (int32(uint32(in[8+inpos])>>10) & 16383)
//
//	out[20+outpos] = int32(uint32(in[8+inpos])>>24) |
//		((in[9+inpos] & 63) << (14 - 6))
//
//	out[21+outpos] = (int32(uint32(in[9+inpos])>>6) & 16383)
//
//	out[22+outpos] = int32(uint32(in[9+inpos])>>20) |
//		((in[10+inpos] & 3) << (14 - 2))
//
//	out[23+outpos] = (int32(uint32(in[10+inpos])>>2) & 16383)
//
//	out[24+outpos] = (int32(uint32(in[10+inpos])>>16) & 16383)
//
//	out[25+outpos] = int32(uint32(in[10+inpos])>>30) |
//		((in[11+inpos] & 4095) << (14 - 12))
//
//	out[26+outpos] = (int32(uint32(in[11+inpos])>>12) & 16383)
//
//	out[27+outpos] = int32(uint32(in[11+inpos])>>26) |
//		((in[12+inpos] & 255) << (14 - 8))
//
//	out[28+outpos] = (int32(uint32(in[12+inpos])>>8) & 16383)
//
//	out[29+outpos] = int32(uint32(in[12+inpos])>>22) |
//		((in[13+inpos] & 15) << (14 - 4))
//
//	out[30+outpos] = (int32(uint32(in[13+inpos])>>4) & 16383)
//
//	out[31+outpos] = int32(uint32(in[13+inpos]) >> 18)
//
//}
//
//func fastunpack15(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 32767)
//
//	out[1+outpos] = (int32(uint32(in[0+inpos])>>15) & 32767)
//
//	out[2+outpos] = int32(uint32(in[0+inpos])>>30) |
//		((in[1+inpos] & 8191) << (15 - 13))
//
//	out[3+outpos] = (int32(uint32(in[1+inpos])>>13) & 32767)
//
//	out[4+outpos] = int32(uint32(in[1+inpos])>>28) |
//		((in[2+inpos] & 2047) << (15 - 11))
//
//	out[5+outpos] = (int32(uint32(in[2+inpos])>>11) & 32767)
//
//	out[6+outpos] = int32(uint32(in[2+inpos])>>26) |
//		((in[3+inpos] & 511) << (15 - 9))
//
//	out[7+outpos] = (int32(uint32(in[3+inpos])>>9) & 32767)
//
//	out[8+outpos] = int32(uint32(in[3+inpos])>>24) |
//		((in[4+inpos] & 127) << (15 - 7))
//
//	out[9+outpos] = (int32(uint32(in[4+inpos])>>7) & 32767)
//
//	out[10+outpos] = int32(uint32(in[4+inpos])>>22) |
//		((in[5+inpos] & 31) << (15 - 5))
//
//	out[11+outpos] = (int32(uint32(in[5+inpos])>>5) & 32767)
//
//	out[12+outpos] = int32(uint32(in[5+inpos])>>20) |
//		((in[6+inpos] & 7) << (15 - 3))
//
//	out[13+outpos] = (int32(uint32(in[6+inpos])>>3) & 32767)
//
//	out[14+outpos] = int32(uint32(in[6+inpos])>>18) |
//		((in[7+inpos] & 1) << (15 - 1))
//
//	out[15+outpos] = (int32(uint32(in[7+inpos])>>1) & 32767)
//
//	out[16+outpos] = (int32(uint32(in[7+inpos])>>16) & 32767)
//
//	out[17+outpos] = int32(uint32(in[7+inpos])>>31) |
//		((in[8+inpos] & 16383) << (15 - 14))
//
//	out[18+outpos] = (int32(uint32(in[8+inpos])>>14) & 32767)
//
//	out[19+outpos] = int32(uint32(in[8+inpos])>>29) |
//		((in[9+inpos] & 4095) << (15 - 12))
//
//	out[20+outpos] = (int32(uint32(in[9+inpos])>>12) & 32767)
//
//	out[21+outpos] = int32(uint32(in[9+inpos])>>27) |
//		((in[10+inpos] & 1023) << (15 - 10))
//
//	out[22+outpos] = (int32(uint32(in[10+inpos])>>10) & 32767)
//
//	out[23+outpos] = int32(uint32(in[10+inpos])>>25) |
//		((in[11+inpos] & 255) << (15 - 8))
//
//	out[24+outpos] = (int32(uint32(in[11+inpos])>>8) & 32767)
//
//	out[25+outpos] = int32(uint32(in[11+inpos])>>23) |
//		((in[12+inpos] & 63) << (15 - 6))
//
//	out[26+outpos] = (int32(uint32(in[12+inpos])>>6) & 32767)
//
//	out[27+outpos] = int32(uint32(in[12+inpos])>>21) |
//		((in[13+inpos] & 15) << (15 - 4))
//
//	out[28+outpos] = (int32(uint32(in[13+inpos])>>4) & 32767)
//
//	out[29+outpos] = int32(uint32(in[13+inpos])>>19) |
//		((in[14+inpos] & 3) << (15 - 2))
//
//	out[30+outpos] = (int32(uint32(in[14+inpos])>>2) & 32767)
//
//	out[31+outpos] = int32(uint32(in[14+inpos]) >> 17)
//
//}
//
//func fastunpack16(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 65535)
//
//	out[1+outpos] = int32(uint32(in[0+inpos]) >> 16)
//
//	out[2+outpos] = (int32(uint32(in[1+inpos])>>0) & 65535)
//
//	out[3+outpos] = int32(uint32(in[1+inpos]) >> 16)
//
//	out[4+outpos] = (int32(uint32(in[2+inpos])>>0) & 65535)
//
//	out[5+outpos] = int32(uint32(in[2+inpos]) >> 16)
//
//	out[6+outpos] = (int32(uint32(in[3+inpos])>>0) & 65535)
//
//	out[7+outpos] = int32(uint32(in[3+inpos]) >> 16)
//
//	out[8+outpos] = (int32(uint32(in[4+inpos])>>0) & 65535)
//
//	out[9+outpos] = int32(uint32(in[4+inpos]) >> 16)
//
//	out[10+outpos] = (int32(uint32(in[5+inpos])>>0) & 65535)
//
//	out[11+outpos] = int32(uint32(in[5+inpos]) >> 16)
//
//	out[12+outpos] = (int32(uint32(in[6+inpos])>>0) & 65535)
//
//	out[13+outpos] = int32(uint32(in[6+inpos]) >> 16)
//
//	out[14+outpos] = (int32(uint32(in[7+inpos])>>0) & 65535)
//
//	out[15+outpos] = int32(uint32(in[7+inpos]) >> 16)
//
//	out[16+outpos] = (int32(uint32(in[8+inpos])>>0) & 65535)
//
//	out[17+outpos] = int32(uint32(in[8+inpos]) >> 16)
//
//	out[18+outpos] = (int32(uint32(in[9+inpos])>>0) & 65535)
//
//	out[19+outpos] = int32(uint32(in[9+inpos]) >> 16)
//
//	out[20+outpos] = (int32(uint32(in[10+inpos])>>0) & 65535)
//
//	out[21+outpos] = int32(uint32(in[10+inpos]) >> 16)
//
//	out[22+outpos] = (int32(uint32(in[11+inpos])>>0) & 65535)
//
//	out[23+outpos] = int32(uint32(in[11+inpos]) >> 16)
//
//	out[24+outpos] = (int32(uint32(in[12+inpos])>>0) & 65535)
//
//	out[25+outpos] = int32(uint32(in[12+inpos]) >> 16)
//
//	out[26+outpos] = (int32(uint32(in[13+inpos])>>0) & 65535)
//
//	out[27+outpos] = int32(uint32(in[13+inpos]) >> 16)
//
//	out[28+outpos] = (int32(uint32(in[14+inpos])>>0) & 65535)
//
//	out[29+outpos] = int32(uint32(in[14+inpos]) >> 16)
//
//	out[30+outpos] = (int32(uint32(in[15+inpos])>>0) & 65535)
//
//	out[31+outpos] = int32(uint32(in[15+inpos]) >> 16)
//
//}
//
//func fastunpack17(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 131071)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>17) |
//		((in[1+inpos] & 3) << (17 - 2))
//
//	out[2+outpos] = (int32(uint32(in[1+inpos])>>2) & 131071)
//
//	out[3+outpos] = int32(uint32(in[1+inpos])>>19) |
//		((in[2+inpos] & 15) << (17 - 4))
//
//	out[4+outpos] = (int32(uint32(in[2+inpos])>>4) & 131071)
//
//	out[5+outpos] = int32(uint32(in[2+inpos])>>21) |
//		((in[3+inpos] & 63) << (17 - 6))
//
//	out[6+outpos] = (int32(uint32(in[3+inpos])>>6) & 131071)
//
//	out[7+outpos] = int32(uint32(in[3+inpos])>>23) |
//		((in[4+inpos] & 255) << (17 - 8))
//
//	out[8+outpos] = (int32(uint32(in[4+inpos])>>8) & 131071)
//
//	out[9+outpos] = int32(uint32(in[4+inpos])>>25) |
//		((in[5+inpos] & 1023) << (17 - 10))
//
//	out[10+outpos] = (int32(uint32(in[5+inpos])>>10) & 131071)
//
//	out[11+outpos] = int32(uint32(in[5+inpos])>>27) |
//		((in[6+inpos] & 4095) << (17 - 12))
//
//	out[12+outpos] = (int32(uint32(in[6+inpos])>>12) & 131071)
//
//	out[13+outpos] = int32(uint32(in[6+inpos])>>29) |
//		((in[7+inpos] & 16383) << (17 - 14))
//
//	out[14+outpos] = (int32(uint32(in[7+inpos])>>14) & 131071)
//
//	out[15+outpos] = int32(uint32(in[7+inpos])>>31) |
//		((in[8+inpos] & 65535) << (17 - 16))
//
//	out[16+outpos] = int32(uint32(in[8+inpos])>>16) |
//		((in[9+inpos] & 1) << (17 - 1))
//
//	out[17+outpos] = (int32(uint32(in[9+inpos])>>1) & 131071)
//
//	out[18+outpos] = int32(uint32(in[9+inpos])>>18) |
//		((in[10+inpos] & 7) << (17 - 3))
//
//	out[19+outpos] = (int32(uint32(in[10+inpos])>>3) & 131071)
//
//	out[20+outpos] = int32(uint32(in[10+inpos])>>20) |
//		((in[11+inpos] & 31) << (17 - 5))
//
//	out[21+outpos] = (int32(uint32(in[11+inpos])>>5) & 131071)
//
//	out[22+outpos] = int32(uint32(in[11+inpos])>>22) |
//		((in[12+inpos] & 127) << (17 - 7))
//
//	out[23+outpos] = (int32(uint32(in[12+inpos])>>7) & 131071)
//
//	out[24+outpos] = int32(uint32(in[12+inpos])>>24) |
//		((in[13+inpos] & 511) << (17 - 9))
//
//	out[25+outpos] = (int32(uint32(in[13+inpos])>>9) & 131071)
//
//	out[26+outpos] = int32(uint32(in[13+inpos])>>26) |
//		((in[14+inpos] & 2047) << (17 - 11))
//
//	out[27+outpos] = (int32(uint32(in[14+inpos])>>11) & 131071)
//
//	out[28+outpos] = int32(uint32(in[14+inpos])>>28) |
//		((in[15+inpos] & 8191) << (17 - 13))
//
//	out[29+outpos] = (int32(uint32(in[15+inpos])>>13) & 131071)
//
//	out[30+outpos] = int32(uint32(in[15+inpos])>>30) |
//		((in[16+inpos] & 32767) << (17 - 15))
//
//	out[31+outpos] = int32(uint32(in[16+inpos]) >> 15)
//
//}
//
//func fastunpack18(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 262143)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>18) |
//		((in[1+inpos] & 15) << (18 - 4))
//
//	out[2+outpos] = (int32(uint32(in[1+inpos])>>4) & 262143)
//
//	out[3+outpos] = int32(uint32(in[1+inpos])>>22) |
//		((in[2+inpos] & 255) << (18 - 8))
//
//	out[4+outpos] = (int32(uint32(in[2+inpos])>>8) & 262143)
//
//	out[5+outpos] = int32(uint32(in[2+inpos])>>26) |
//		((in[3+inpos] & 4095) << (18 - 12))
//
//	out[6+outpos] = (int32(uint32(in[3+inpos])>>12) & 262143)
//
//	out[7+outpos] = int32(uint32(in[3+inpos])>>30) |
//		((in[4+inpos] & 65535) << (18 - 16))
//
//	out[8+outpos] = int32(uint32(in[4+inpos])>>16) |
//		((in[5+inpos] & 3) << (18 - 2))
//
//	out[9+outpos] = (int32(uint32(in[5+inpos])>>2) & 262143)
//
//	out[10+outpos] = int32(uint32(in[5+inpos])>>20) |
//		((in[6+inpos] & 63) << (18 - 6))
//
//	out[11+outpos] = (int32(uint32(in[6+inpos])>>6) & 262143)
//
//	out[12+outpos] = int32(uint32(in[6+inpos])>>24) |
//		((in[7+inpos] & 1023) << (18 - 10))
//
//	out[13+outpos] = (int32(uint32(in[7+inpos])>>10) & 262143)
//
//	out[14+outpos] = int32(uint32(in[7+inpos])>>28) |
//		((in[8+inpos] & 16383) << (18 - 14))
//
//	out[15+outpos] = int32(uint32(in[8+inpos]) >> 14)
//
//	out[16+outpos] = (int32(uint32(in[9+inpos])>>0) & 262143)
//
//	out[17+outpos] = int32(uint32(in[9+inpos])>>18) |
//		((in[10+inpos] & 15) << (18 - 4))
//
//	out[18+outpos] = (int32(uint32(in[10+inpos])>>4) & 262143)
//
//	out[19+outpos] = int32(uint32(in[10+inpos])>>22) |
//		((in[11+inpos] & 255) << (18 - 8))
//
//	out[20+outpos] = (int32(uint32(in[11+inpos])>>8) & 262143)
//
//	out[21+outpos] = int32(uint32(in[11+inpos])>>26) |
//		((in[12+inpos] & 4095) << (18 - 12))
//
//	out[22+outpos] = (int32(uint32(in[12+inpos])>>12) & 262143)
//
//	out[23+outpos] = int32(uint32(in[12+inpos])>>30) |
//		((in[13+inpos] & 65535) << (18 - 16))
//
//	out[24+outpos] = int32(uint32(in[13+inpos])>>16) |
//		((in[14+inpos] & 3) << (18 - 2))
//
//	out[25+outpos] = (int32(uint32(in[14+inpos])>>2) & 262143)
//
//	out[26+outpos] = int32(uint32(in[14+inpos])>>20) |
//		((in[15+inpos] & 63) << (18 - 6))
//
//	out[27+outpos] = (int32(uint32(in[15+inpos])>>6) & 262143)
//
//	out[28+outpos] = int32(uint32(in[15+inpos])>>24) |
//		((in[16+inpos] & 1023) << (18 - 10))
//
//	out[29+outpos] = (int32(uint32(in[16+inpos])>>10) & 262143)
//
//	out[30+outpos] = int32(uint32(in[16+inpos])>>28) |
//		((in[17+inpos] & 16383) << (18 - 14))
//
//	out[31+outpos] = int32(uint32(in[17+inpos]) >> 14)
//
//}
//
//func fastunpack19(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 524287)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>19) |
//		((in[1+inpos] & 63) << (19 - 6))
//
//	out[2+outpos] = (int32(uint32(in[1+inpos])>>6) & 524287)
//
//	out[3+outpos] = int32(uint32(in[1+inpos])>>25) |
//		((in[2+inpos] & 4095) << (19 - 12))
//
//	out[4+outpos] = (int32(uint32(in[2+inpos])>>12) & 524287)
//
//	out[5+outpos] = int32(uint32(in[2+inpos])>>31) |
//		((in[3+inpos] & 262143) << (19 - 18))
//
//	out[6+outpos] = int32(uint32(in[3+inpos])>>18) |
//		((in[4+inpos] & 31) << (19 - 5))
//
//	out[7+outpos] = (int32(uint32(in[4+inpos])>>5) & 524287)
//
//	out[8+outpos] = int32(uint32(in[4+inpos])>>24) |
//		((in[5+inpos] & 2047) << (19 - 11))
//
//	out[9+outpos] = (int32(uint32(in[5+inpos])>>11) & 524287)
//
//	out[10+outpos] = int32(uint32(in[5+inpos])>>30) |
//		((in[6+inpos] & 131071) << (19 - 17))
//
//	out[11+outpos] = int32(uint32(in[6+inpos])>>17) |
//		((in[7+inpos] & 15) << (19 - 4))
//
//	out[12+outpos] = (int32(uint32(in[7+inpos])>>4) & 524287)
//
//	out[13+outpos] = int32(uint32(in[7+inpos])>>23) |
//		((in[8+inpos] & 1023) << (19 - 10))
//
//	out[14+outpos] = (int32(uint32(in[8+inpos])>>10) & 524287)
//
//	out[15+outpos] = int32(uint32(in[8+inpos])>>29) |
//		((in[9+inpos] & 65535) << (19 - 16))
//
//	out[16+outpos] = int32(uint32(in[9+inpos])>>16) |
//		((in[10+inpos] & 7) << (19 - 3))
//
//	out[17+outpos] = (int32(uint32(in[10+inpos])>>3) & 524287)
//
//	out[18+outpos] = int32(uint32(in[10+inpos])>>22) |
//		((in[11+inpos] & 511) << (19 - 9))
//
//	out[19+outpos] = (int32(uint32(in[11+inpos])>>9) & 524287)
//
//	out[20+outpos] = int32(uint32(in[11+inpos])>>28) |
//		((in[12+inpos] & 32767) << (19 - 15))
//
//	out[21+outpos] = int32(uint32(in[12+inpos])>>15) |
//		((in[13+inpos] & 3) << (19 - 2))
//
//	out[22+outpos] = (int32(uint32(in[13+inpos])>>2) & 524287)
//
//	out[23+outpos] = int32(uint32(in[13+inpos])>>21) |
//		((in[14+inpos] & 255) << (19 - 8))
//
//	out[24+outpos] = (int32(uint32(in[14+inpos])>>8) & 524287)
//
//	out[25+outpos] = int32(uint32(in[14+inpos])>>27) |
//		((in[15+inpos] & 16383) << (19 - 14))
//
//	out[26+outpos] = int32(uint32(in[15+inpos])>>14) |
//		((in[16+inpos] & 1) << (19 - 1))
//
//	out[27+outpos] = (int32(uint32(in[16+inpos])>>1) & 524287)
//
//	out[28+outpos] = int32(uint32(in[16+inpos])>>20) |
//		((in[17+inpos] & 127) << (19 - 7))
//
//	out[29+outpos] = (int32(uint32(in[17+inpos])>>7) & 524287)
//
//	out[30+outpos] = int32(uint32(in[17+inpos])>>26) |
//		((in[18+inpos] & 8191) << (19 - 13))
//
//	out[31+outpos] = int32(uint32(in[18+inpos]) >> 13)
//
//}
//
//func fastunpack20(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 1048575)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>20) |
//		((in[1+inpos] & 255) << (20 - 8))
//
//	out[2+outpos] = (int32(uint32(in[1+inpos])>>8) & 1048575)
//
//	out[3+outpos] = int32(uint32(in[1+inpos])>>28) |
//		((in[2+inpos] & 65535) << (20 - 16))
//
//	out[4+outpos] = int32(uint32(in[2+inpos])>>16) |
//		((in[3+inpos] & 15) << (20 - 4))
//
//	out[5+outpos] = (int32(uint32(in[3+inpos])>>4) & 1048575)
//
//	out[6+outpos] = int32(uint32(in[3+inpos])>>24) |
//		((in[4+inpos] & 4095) << (20 - 12))
//
//	out[7+outpos] = int32(uint32(in[4+inpos]) >> 12)
//
//	out[8+outpos] = (int32(uint32(in[5+inpos])>>0) & 1048575)
//
//	out[9+outpos] = int32(uint32(in[5+inpos])>>20) |
//		((in[6+inpos] & 255) << (20 - 8))
//
//	out[10+outpos] = (int32(uint32(in[6+inpos])>>8) & 1048575)
//
//	out[11+outpos] = int32(uint32(in[6+inpos])>>28) |
//		((in[7+inpos] & 65535) << (20 - 16))
//
//	out[12+outpos] = int32(uint32(in[7+inpos])>>16) |
//		((in[8+inpos] & 15) << (20 - 4))
//
//	out[13+outpos] = (int32(uint32(in[8+inpos])>>4) & 1048575)
//
//	out[14+outpos] = int32(uint32(in[8+inpos])>>24) |
//		((in[9+inpos] & 4095) << (20 - 12))
//
//	out[15+outpos] = int32(uint32(in[9+inpos]) >> 12)
//
//	out[16+outpos] = (int32(uint32(in[10+inpos])>>0) & 1048575)
//
//	out[17+outpos] = int32(uint32(in[10+inpos])>>20) |
//		((in[11+inpos] & 255) << (20 - 8))
//
//	out[18+outpos] = (int32(uint32(in[11+inpos])>>8) & 1048575)
//
//	out[19+outpos] = int32(uint32(in[11+inpos])>>28) |
//		((in[12+inpos] & 65535) << (20 - 16))
//
//	out[20+outpos] = int32(uint32(in[12+inpos])>>16) |
//		((in[13+inpos] & 15) << (20 - 4))
//
//	out[21+outpos] = (int32(uint32(in[13+inpos])>>4) & 1048575)
//
//	out[22+outpos] = int32(uint32(in[13+inpos])>>24) |
//		((in[14+inpos] & 4095) << (20 - 12))
//
//	out[23+outpos] = int32(uint32(in[14+inpos]) >> 12)
//
//	out[24+outpos] = (int32(uint32(in[15+inpos])>>0) & 1048575)
//
//	out[25+outpos] = int32(uint32(in[15+inpos])>>20) |
//		((in[16+inpos] & 255) << (20 - 8))
//
//	out[26+outpos] = (int32(uint32(in[16+inpos])>>8) & 1048575)
//
//	out[27+outpos] = int32(uint32(in[16+inpos])>>28) |
//		((in[17+inpos] & 65535) << (20 - 16))
//
//	out[28+outpos] = int32(uint32(in[17+inpos])>>16) |
//		((in[18+inpos] & 15) << (20 - 4))
//
//	out[29+outpos] = (int32(uint32(in[18+inpos])>>4) & 1048575)
//
//	out[30+outpos] = int32(uint32(in[18+inpos])>>24) |
//		((in[19+inpos] & 4095) << (20 - 12))
//
//	out[31+outpos] = int32(uint32(in[19+inpos]) >> 12)
//
//}
//
//func fastunpack21(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 2097151)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>21) |
//		((in[1+inpos] & 1023) << (21 - 10))
//
//	out[2+outpos] = (int32(uint32(in[1+inpos])>>10) & 2097151)
//
//	out[3+outpos] = int32(uint32(in[1+inpos])>>31) |
//		((in[2+inpos] & 1048575) << (21 - 20))
//
//	out[4+outpos] = int32(uint32(in[2+inpos])>>20) |
//		((in[3+inpos] & 511) << (21 - 9))
//
//	out[5+outpos] = (int32(uint32(in[3+inpos])>>9) & 2097151)
//
//	out[6+outpos] = int32(uint32(in[3+inpos])>>30) |
//		((in[4+inpos] & 524287) << (21 - 19))
//
//	out[7+outpos] = int32(uint32(in[4+inpos])>>19) |
//		((in[5+inpos] & 255) << (21 - 8))
//
//	out[8+outpos] = (int32(uint32(in[5+inpos])>>8) & 2097151)
//
//	out[9+outpos] = int32(uint32(in[5+inpos])>>29) |
//		((in[6+inpos] & 262143) << (21 - 18))
//
//	out[10+outpos] = int32(uint32(in[6+inpos])>>18) |
//		((in[7+inpos] & 127) << (21 - 7))
//
//	out[11+outpos] = (int32(uint32(in[7+inpos])>>7) & 2097151)
//
//	out[12+outpos] = int32(uint32(in[7+inpos])>>28) |
//		((in[8+inpos] & 131071) << (21 - 17))
//
//	out[13+outpos] = int32(uint32(in[8+inpos])>>17) |
//		((in[9+inpos] & 63) << (21 - 6))
//
//	out[14+outpos] = (int32(uint32(in[9+inpos])>>6) & 2097151)
//
//	out[15+outpos] = int32(uint32(in[9+inpos])>>27) |
//		((in[10+inpos] & 65535) << (21 - 16))
//
//	out[16+outpos] = int32(uint32(in[10+inpos])>>16) |
//		((in[11+inpos] & 31) << (21 - 5))
//
//	out[17+outpos] = (int32(uint32(in[11+inpos])>>5) & 2097151)
//
//	out[18+outpos] = int32(uint32(in[11+inpos])>>26) |
//		((in[12+inpos] & 32767) << (21 - 15))
//
//	out[19+outpos] = int32(uint32(in[12+inpos])>>15) |
//		((in[13+inpos] & 15) << (21 - 4))
//
//	out[20+outpos] = (int32(uint32(in[13+inpos])>>4) & 2097151)
//
//	out[21+outpos] = int32(uint32(in[13+inpos])>>25) |
//		((in[14+inpos] & 16383) << (21 - 14))
//
//	out[22+outpos] = int32(uint32(in[14+inpos])>>14) |
//		((in[15+inpos] & 7) << (21 - 3))
//
//	out[23+outpos] = (int32(uint32(in[15+inpos])>>3) & 2097151)
//
//	out[24+outpos] = int32(uint32(in[15+inpos])>>24) |
//		((in[16+inpos] & 8191) << (21 - 13))
//
//	out[25+outpos] = int32(uint32(in[16+inpos])>>13) |
//		((in[17+inpos] & 3) << (21 - 2))
//
//	out[26+outpos] = (int32(uint32(in[17+inpos])>>2) & 2097151)
//
//	out[27+outpos] = int32(uint32(in[17+inpos])>>23) |
//		((in[18+inpos] & 4095) << (21 - 12))
//
//	out[28+outpos] = int32(uint32(in[18+inpos])>>12) |
//		((in[19+inpos] & 1) << (21 - 1))
//
//	out[29+outpos] = (int32(uint32(in[19+inpos])>>1) & 2097151)
//
//	out[30+outpos] = int32(uint32(in[19+inpos])>>22) |
//		((in[20+inpos] & 2047) << (21 - 11))
//
//	out[31+outpos] = int32(uint32(in[20+inpos]) >> 11)
//
//}
//
//func fastunpack22(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 4194303)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>22) |
//		((in[1+inpos] & 4095) << (22 - 12))
//
//	out[2+outpos] = int32(uint32(in[1+inpos])>>12) |
//		((in[2+inpos] & 3) << (22 - 2))
//
//	out[3+outpos] = (int32(uint32(in[2+inpos])>>2) & 4194303)
//
//	out[4+outpos] = int32(uint32(in[2+inpos])>>24) |
//		((in[3+inpos] & 16383) << (22 - 14))
//
//	out[5+outpos] = int32(uint32(in[3+inpos])>>14) |
//		((in[4+inpos] & 15) << (22 - 4))
//
//	out[6+outpos] = (int32(uint32(in[4+inpos])>>4) & 4194303)
//
//	out[7+outpos] = int32(uint32(in[4+inpos])>>26) |
//		((in[5+inpos] & 65535) << (22 - 16))
//
//	out[8+outpos] = int32(uint32(in[5+inpos])>>16) |
//		((in[6+inpos] & 63) << (22 - 6))
//
//	out[9+outpos] = (int32(uint32(in[6+inpos])>>6) & 4194303)
//
//	out[10+outpos] = int32(uint32(in[6+inpos])>>28) |
//		((in[7+inpos] & 262143) << (22 - 18))
//
//	out[11+outpos] = int32(uint32(in[7+inpos])>>18) |
//		((in[8+inpos] & 255) << (22 - 8))
//
//	out[12+outpos] = (int32(uint32(in[8+inpos])>>8) & 4194303)
//
//	out[13+outpos] = int32(uint32(in[8+inpos])>>30) |
//		((in[9+inpos] & 1048575) << (22 - 20))
//
//	out[14+outpos] = int32(uint32(in[9+inpos])>>20) |
//		((in[10+inpos] & 1023) << (22 - 10))
//
//	out[15+outpos] = int32(uint32(in[10+inpos]) >> 10)
//
//	out[16+outpos] = (int32(uint32(in[11+inpos])>>0) & 4194303)
//
//	out[17+outpos] = int32(uint32(in[11+inpos])>>22) |
//		((in[12+inpos] & 4095) << (22 - 12))
//
//	out[18+outpos] = int32(uint32(in[12+inpos])>>12) |
//		((in[13+inpos] & 3) << (22 - 2))
//
//	out[19+outpos] = (int32(uint32(in[13+inpos])>>2) & 4194303)
//
//	out[20+outpos] = int32(uint32(in[13+inpos])>>24) |
//		((in[14+inpos] & 16383) << (22 - 14))
//
//	out[21+outpos] = int32(uint32(in[14+inpos])>>14) |
//		((in[15+inpos] & 15) << (22 - 4))
//
//	out[22+outpos] = (int32(uint32(in[15+inpos])>>4) & 4194303)
//
//	out[23+outpos] = int32(uint32(in[15+inpos])>>26) |
//		((in[16+inpos] & 65535) << (22 - 16))
//
//	out[24+outpos] = int32(uint32(in[16+inpos])>>16) |
//		((in[17+inpos] & 63) << (22 - 6))
//
//	out[25+outpos] = (int32(uint32(in[17+inpos])>>6) & 4194303)
//
//	out[26+outpos] = int32(uint32(in[17+inpos])>>28) |
//		((in[18+inpos] & 262143) << (22 - 18))
//
//	out[27+outpos] = int32(uint32(in[18+inpos])>>18) |
//		((in[19+inpos] & 255) << (22 - 8))
//
//	out[28+outpos] = (int32(uint32(in[19+inpos])>>8) & 4194303)
//
//	out[29+outpos] = int32(uint32(in[19+inpos])>>30) |
//		((in[20+inpos] & 1048575) << (22 - 20))
//
//	out[30+outpos] = int32(uint32(in[20+inpos])>>20) |
//		((in[21+inpos] & 1023) << (22 - 10))
//
//	out[31+outpos] = int32(uint32(in[21+inpos]) >> 10)
//
//}
//
//func fastunpack23(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 8388607)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>23) |
//		((in[1+inpos] & 16383) << (23 - 14))
//
//	out[2+outpos] = int32(uint32(in[1+inpos])>>14) |
//		((in[2+inpos] & 31) << (23 - 5))
//
//	out[3+outpos] = (int32(uint32(in[2+inpos])>>5) & 8388607)
//
//	out[4+outpos] = int32(uint32(in[2+inpos])>>28) |
//		((in[3+inpos] & 524287) << (23 - 19))
//
//	out[5+outpos] = int32(uint32(in[3+inpos])>>19) |
//		((in[4+inpos] & 1023) << (23 - 10))
//
//	out[6+outpos] = int32(uint32(in[4+inpos])>>10) |
//		((in[5+inpos] & 1) << (23 - 1))
//
//	out[7+outpos] = (int32(uint32(in[5+inpos])>>1) & 8388607)
//
//	out[8+outpos] = int32(uint32(in[5+inpos])>>24) |
//		((in[6+inpos] & 32767) << (23 - 15))
//
//	out[9+outpos] = int32(uint32(in[6+inpos])>>15) |
//		((in[7+inpos] & 63) << (23 - 6))
//
//	out[10+outpos] = (int32(uint32(in[7+inpos])>>6) & 8388607)
//
//	out[11+outpos] = int32(uint32(in[7+inpos])>>29) |
//		((in[8+inpos] & 1048575) << (23 - 20))
//
//	out[12+outpos] = int32(uint32(in[8+inpos])>>20) |
//		((in[9+inpos] & 2047) << (23 - 11))
//
//	out[13+outpos] = int32(uint32(in[9+inpos])>>11) |
//		((in[10+inpos] & 3) << (23 - 2))
//
//	out[14+outpos] = (int32(uint32(in[10+inpos])>>2) & 8388607)
//
//	out[15+outpos] = int32(uint32(in[10+inpos])>>25) |
//		((in[11+inpos] & 65535) << (23 - 16))
//
//	out[16+outpos] = int32(uint32(in[11+inpos])>>16) |
//		((in[12+inpos] & 127) << (23 - 7))
//
//	out[17+outpos] = (int32(uint32(in[12+inpos])>>7) & 8388607)
//
//	out[18+outpos] = int32(uint32(in[12+inpos])>>30) |
//		((in[13+inpos] & 2097151) << (23 - 21))
//
//	out[19+outpos] = int32(uint32(in[13+inpos])>>21) |
//		((in[14+inpos] & 4095) << (23 - 12))
//
//	out[20+outpos] = int32(uint32(in[14+inpos])>>12) |
//		((in[15+inpos] & 7) << (23 - 3))
//
//	out[21+outpos] = (int32(uint32(in[15+inpos])>>3) & 8388607)
//
//	out[22+outpos] = int32(uint32(in[15+inpos])>>26) |
//		((in[16+inpos] & 131071) << (23 - 17))
//
//	out[23+outpos] = int32(uint32(in[16+inpos])>>17) |
//		((in[17+inpos] & 255) << (23 - 8))
//
//	out[24+outpos] = (int32(uint32(in[17+inpos])>>8) & 8388607)
//
//	out[25+outpos] = int32(uint32(in[17+inpos])>>31) |
//		((in[18+inpos] & 4194303) << (23 - 22))
//
//	out[26+outpos] = int32(uint32(in[18+inpos])>>22) |
//		((in[19+inpos] & 8191) << (23 - 13))
//
//	out[27+outpos] = int32(uint32(in[19+inpos])>>13) |
//		((in[20+inpos] & 15) << (23 - 4))
//
//	out[28+outpos] = (int32(uint32(in[20+inpos])>>4) & 8388607)
//
//	out[29+outpos] = int32(uint32(in[20+inpos])>>27) |
//		((in[21+inpos] & 262143) << (23 - 18))
//
//	out[30+outpos] = int32(uint32(in[21+inpos])>>18) |
//		((in[22+inpos] & 511) << (23 - 9))
//
//	out[31+outpos] = int32(uint32(in[22+inpos]) >> 9)
//
//}
//
//func fastunpack24(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 16777215)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>24) |
//		((in[1+inpos] & 65535) << (24 - 16))
//
//	out[2+outpos] = int32(uint32(in[1+inpos])>>16) |
//		((in[2+inpos] & 255) << (24 - 8))
//
//	out[3+outpos] = int32(uint32(in[2+inpos]) >> 8)
//
//	out[4+outpos] = (int32(uint32(in[3+inpos])>>0) & 16777215)
//
//	out[5+outpos] = int32(uint32(in[3+inpos])>>24) |
//		((in[4+inpos] & 65535) << (24 - 16))
//
//	out[6+outpos] = int32(uint32(in[4+inpos])>>16) |
//		((in[5+inpos] & 255) << (24 - 8))
//
//	out[7+outpos] = int32(uint32(in[5+inpos]) >> 8)
//
//	out[8+outpos] = (int32(uint32(in[6+inpos])>>0) & 16777215)
//
//	out[9+outpos] = int32(uint32(in[6+inpos])>>24) |
//		((in[7+inpos] & 65535) << (24 - 16))
//
//	out[10+outpos] = int32(uint32(in[7+inpos])>>16) |
//		((in[8+inpos] & 255) << (24 - 8))
//
//	out[11+outpos] = int32(uint32(in[8+inpos]) >> 8)
//
//	out[12+outpos] = (int32(uint32(in[9+inpos])>>0) & 16777215)
//
//	out[13+outpos] = int32(uint32(in[9+inpos])>>24) |
//		((in[10+inpos] & 65535) << (24 - 16))
//
//	out[14+outpos] = int32(uint32(in[10+inpos])>>16) |
//		((in[11+inpos] & 255) << (24 - 8))
//
//	out[15+outpos] = int32(uint32(in[11+inpos]) >> 8)
//
//	out[16+outpos] = (int32(uint32(in[12+inpos])>>0) & 16777215)
//
//	out[17+outpos] = int32(uint32(in[12+inpos])>>24) |
//		((in[13+inpos] & 65535) << (24 - 16))
//
//	out[18+outpos] = int32(uint32(in[13+inpos])>>16) |
//		((in[14+inpos] & 255) << (24 - 8))
//
//	out[19+outpos] = int32(uint32(in[14+inpos]) >> 8)
//
//	out[20+outpos] = (int32(uint32(in[15+inpos])>>0) & 16777215)
//
//	out[21+outpos] = int32(uint32(in[15+inpos])>>24) |
//		((in[16+inpos] & 65535) << (24 - 16))
//
//	out[22+outpos] = int32(uint32(in[16+inpos])>>16) |
//		((in[17+inpos] & 255) << (24 - 8))
//
//	out[23+outpos] = int32(uint32(in[17+inpos]) >> 8)
//
//	out[24+outpos] = (int32(uint32(in[18+inpos])>>0) & 16777215)
//
//	out[25+outpos] = int32(uint32(in[18+inpos])>>24) |
//		((in[19+inpos] & 65535) << (24 - 16))
//
//	out[26+outpos] = int32(uint32(in[19+inpos])>>16) |
//		((in[20+inpos] & 255) << (24 - 8))
//
//	out[27+outpos] = int32(uint32(in[20+inpos]) >> 8)
//
//	out[28+outpos] = (int32(uint32(in[21+inpos])>>0) & 16777215)
//
//	out[29+outpos] = int32(uint32(in[21+inpos])>>24) |
//		((in[22+inpos] & 65535) << (24 - 16))
//
//	out[30+outpos] = int32(uint32(in[22+inpos])>>16) |
//		((in[23+inpos] & 255) << (24 - 8))
//
//	out[31+outpos] = int32(uint32(in[23+inpos]) >> 8)
//
//}
//
//func fastunpack25(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 33554431)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>25) |
//		((in[1+inpos] & 262143) << (25 - 18))
//
//	out[2+outpos] = int32(uint32(in[1+inpos])>>18) |
//		((in[2+inpos] & 2047) << (25 - 11))
//
//	out[3+outpos] = int32(uint32(in[2+inpos])>>11) |
//		((in[3+inpos] & 15) << (25 - 4))
//
//	out[4+outpos] = (int32(uint32(in[3+inpos])>>4) & 33554431)
//
//	out[5+outpos] = int32(uint32(in[3+inpos])>>29) |
//		((in[4+inpos] & 4194303) << (25 - 22))
//
//	out[6+outpos] = int32(uint32(in[4+inpos])>>22) |
//		((in[5+inpos] & 32767) << (25 - 15))
//
//	out[7+outpos] = int32(uint32(in[5+inpos])>>15) |
//		((in[6+inpos] & 255) << (25 - 8))
//
//	out[8+outpos] = int32(uint32(in[6+inpos])>>8) |
//		((in[7+inpos] & 1) << (25 - 1))
//
//	out[9+outpos] = (int32(uint32(in[7+inpos])>>1) & 33554431)
//
//	out[10+outpos] = int32(uint32(in[7+inpos])>>26) |
//		((in[8+inpos] & 524287) << (25 - 19))
//
//	out[11+outpos] = int32(uint32(in[8+inpos])>>19) |
//		((in[9+inpos] & 4095) << (25 - 12))
//
//	out[12+outpos] = int32(uint32(in[9+inpos])>>12) |
//		((in[10+inpos] & 31) << (25 - 5))
//
//	out[13+outpos] = (int32(uint32(in[10+inpos])>>5) & 33554431)
//
//	out[14+outpos] = int32(uint32(in[10+inpos])>>30) |
//		((in[11+inpos] & 8388607) << (25 - 23))
//
//	out[15+outpos] = int32(uint32(in[11+inpos])>>23) |
//		((in[12+inpos] & 65535) << (25 - 16))
//
//	out[16+outpos] = int32(uint32(in[12+inpos])>>16) |
//		((in[13+inpos] & 511) << (25 - 9))
//
//	out[17+outpos] = int32(uint32(in[13+inpos])>>9) |
//		((in[14+inpos] & 3) << (25 - 2))
//
//	out[18+outpos] = (int32(uint32(in[14+inpos])>>2) & 33554431)
//
//	out[19+outpos] = int32(uint32(in[14+inpos])>>27) |
//		((in[15+inpos] & 1048575) << (25 - 20))
//
//	out[20+outpos] = int32(uint32(in[15+inpos])>>20) |
//		((in[16+inpos] & 8191) << (25 - 13))
//
//	out[21+outpos] = int32(uint32(in[16+inpos])>>13) |
//		((in[17+inpos] & 63) << (25 - 6))
//
//	out[22+outpos] = (int32(uint32(in[17+inpos])>>6) & 33554431)
//
//	out[23+outpos] = int32(uint32(in[17+inpos])>>31) |
//		((in[18+inpos] & 16777215) << (25 - 24))
//
//	out[24+outpos] = int32(uint32(in[18+inpos])>>24) |
//		((in[19+inpos] & 131071) << (25 - 17))
//
//	out[25+outpos] = int32(uint32(in[19+inpos])>>17) |
//		((in[20+inpos] & 1023) << (25 - 10))
//
//	out[26+outpos] = int32(uint32(in[20+inpos])>>10) |
//		((in[21+inpos] & 7) << (25 - 3))
//
//	out[27+outpos] = (int32(uint32(in[21+inpos])>>3) & 33554431)
//
//	out[28+outpos] = int32(uint32(in[21+inpos])>>28) |
//		((in[22+inpos] & 2097151) << (25 - 21))
//
//	out[29+outpos] = int32(uint32(in[22+inpos])>>21) |
//		((in[23+inpos] & 16383) << (25 - 14))
//
//	out[30+outpos] = int32(uint32(in[23+inpos])>>14) |
//		((in[24+inpos] & 127) << (25 - 7))
//
//	out[31+outpos] = int32(uint32(in[24+inpos]) >> 7)
//
//}
//
//func fastunpack26(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 67108863)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>26) |
//		((in[1+inpos] & 1048575) << (26 - 20))
//
//	out[2+outpos] = int32(uint32(in[1+inpos])>>20) |
//		((in[2+inpos] & 16383) << (26 - 14))
//
//	out[3+outpos] = int32(uint32(in[2+inpos])>>14) |
//		((in[3+inpos] & 255) << (26 - 8))
//
//	out[4+outpos] = int32(uint32(in[3+inpos])>>8) |
//		((in[4+inpos] & 3) << (26 - 2))
//
//	out[5+outpos] = (int32(uint32(in[4+inpos])>>2) & 67108863)
//
//	out[6+outpos] = int32(uint32(in[4+inpos])>>28) |
//		((in[5+inpos] & 4194303) << (26 - 22))
//
//	out[7+outpos] = int32(uint32(in[5+inpos])>>22) |
//		((in[6+inpos] & 65535) << (26 - 16))
//
//	out[8+outpos] = int32(uint32(in[6+inpos])>>16) |
//		((in[7+inpos] & 1023) << (26 - 10))
//
//	out[9+outpos] = int32(uint32(in[7+inpos])>>10) |
//		((in[8+inpos] & 15) << (26 - 4))
//
//	out[10+outpos] = (int32(uint32(in[8+inpos])>>4) & 67108863)
//
//	out[11+outpos] = int32(uint32(in[8+inpos])>>30) |
//		((in[9+inpos] & 16777215) << (26 - 24))
//
//	out[12+outpos] = int32(uint32(in[9+inpos])>>24) |
//		((in[10+inpos] & 262143) << (26 - 18))
//
//	out[13+outpos] = int32(uint32(in[10+inpos])>>18) |
//		((in[11+inpos] & 4095) << (26 - 12))
//
//	out[14+outpos] = int32(uint32(in[11+inpos])>>12) |
//		((in[12+inpos] & 63) << (26 - 6))
//
//	out[15+outpos] = int32(uint32(in[12+inpos]) >> 6)
//
//	out[16+outpos] = (int32(uint32(in[13+inpos])>>0) & 67108863)
//
//	out[17+outpos] = int32(uint32(in[13+inpos])>>26) |
//		((in[14+inpos] & 1048575) << (26 - 20))
//
//	out[18+outpos] = int32(uint32(in[14+inpos])>>20) |
//		((in[15+inpos] & 16383) << (26 - 14))
//
//	out[19+outpos] = int32(uint32(in[15+inpos])>>14) |
//		((in[16+inpos] & 255) << (26 - 8))
//
//	out[20+outpos] = int32(uint32(in[16+inpos])>>8) |
//		((in[17+inpos] & 3) << (26 - 2))
//
//	out[21+outpos] = (int32(uint32(in[17+inpos])>>2) & 67108863)
//
//	out[22+outpos] = int32(uint32(in[17+inpos])>>28) |
//		((in[18+inpos] & 4194303) << (26 - 22))
//
//	out[23+outpos] = int32(uint32(in[18+inpos])>>22) |
//		((in[19+inpos] & 65535) << (26 - 16))
//
//	out[24+outpos] = int32(uint32(in[19+inpos])>>16) |
//		((in[20+inpos] & 1023) << (26 - 10))
//
//	out[25+outpos] = int32(uint32(in[20+inpos])>>10) |
//		((in[21+inpos] & 15) << (26 - 4))
//
//	out[26+outpos] = (int32(uint32(in[21+inpos])>>4) & 67108863)
//
//	out[27+outpos] = int32(uint32(in[21+inpos])>>30) |
//		((in[22+inpos] & 16777215) << (26 - 24))
//
//	out[28+outpos] = int32(uint32(in[22+inpos])>>24) |
//		((in[23+inpos] & 262143) << (26 - 18))
//
//	out[29+outpos] = int32(uint32(in[23+inpos])>>18) |
//		((in[24+inpos] & 4095) << (26 - 12))
//
//	out[30+outpos] = int32(uint32(in[24+inpos])>>12) |
//		((in[25+inpos] & 63) << (26 - 6))
//
//	out[31+outpos] = int32(uint32(in[25+inpos]) >> 6)
//
//}
//
//func fastunpack27(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 134217727)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>27) |
//		((in[1+inpos] & 4194303) << (27 - 22))
//
//	out[2+outpos] = int32(uint32(in[1+inpos])>>22) |
//		((in[2+inpos] & 131071) << (27 - 17))
//
//	out[3+outpos] = int32(uint32(in[2+inpos])>>17) |
//		((in[3+inpos] & 4095) << (27 - 12))
//
//	out[4+outpos] = int32(uint32(in[3+inpos])>>12) |
//		((in[4+inpos] & 127) << (27 - 7))
//
//	out[5+outpos] = int32(uint32(in[4+inpos])>>7) |
//		((in[5+inpos] & 3) << (27 - 2))
//
//	out[6+outpos] = (int32(uint32(in[5+inpos])>>2) & 134217727)
//
//	out[7+outpos] = int32(uint32(in[5+inpos])>>29) |
//		((in[6+inpos] & 16777215) << (27 - 24))
//
//	out[8+outpos] = int32(uint32(in[6+inpos])>>24) |
//		((in[7+inpos] & 524287) << (27 - 19))
//
//	out[9+outpos] = int32(uint32(in[7+inpos])>>19) |
//		((in[8+inpos] & 16383) << (27 - 14))
//
//	out[10+outpos] = int32(uint32(in[8+inpos])>>14) |
//		((in[9+inpos] & 511) << (27 - 9))
//
//	out[11+outpos] = int32(uint32(in[9+inpos])>>9) |
//		((in[10+inpos] & 15) << (27 - 4))
//
//	out[12+outpos] = (int32(uint32(in[10+inpos])>>4) & 134217727)
//
//	out[13+outpos] = int32(uint32(in[10+inpos])>>31) |
//		((in[11+inpos] & 67108863) << (27 - 26))
//
//	out[14+outpos] = int32(uint32(in[11+inpos])>>26) |
//		((in[12+inpos] & 2097151) << (27 - 21))
//
//	out[15+outpos] = int32(uint32(in[12+inpos])>>21) |
//		((in[13+inpos] & 65535) << (27 - 16))
//
//	out[16+outpos] = int32(uint32(in[13+inpos])>>16) |
//		((in[14+inpos] & 2047) << (27 - 11))
//
//	out[17+outpos] = int32(uint32(in[14+inpos])>>11) |
//		((in[15+inpos] & 63) << (27 - 6))
//
//	out[18+outpos] = int32(uint32(in[15+inpos])>>6) |
//		((in[16+inpos] & 1) << (27 - 1))
//
//	out[19+outpos] = (int32(uint32(in[16+inpos])>>1) & 134217727)
//
//	out[20+outpos] = int32(uint32(in[16+inpos])>>28) |
//		((in[17+inpos] & 8388607) << (27 - 23))
//
//	out[21+outpos] = int32(uint32(in[17+inpos])>>23) |
//		((in[18+inpos] & 262143) << (27 - 18))
//
//	out[22+outpos] = int32(uint32(in[18+inpos])>>18) |
//		((in[19+inpos] & 8191) << (27 - 13))
//
//	out[23+outpos] = int32(uint32(in[19+inpos])>>13) |
//		((in[20+inpos] & 255) << (27 - 8))
//
//	out[24+outpos] = int32(uint32(in[20+inpos])>>8) |
//		((in[21+inpos] & 7) << (27 - 3))
//
//	out[25+outpos] = (int32(uint32(in[21+inpos])>>3) & 134217727)
//
//	out[26+outpos] = int32(uint32(in[21+inpos])>>30) |
//		((in[22+inpos] & 33554431) << (27 - 25))
//
//	out[27+outpos] = int32(uint32(in[22+inpos])>>25) |
//		((in[23+inpos] & 1048575) << (27 - 20))
//
//	out[28+outpos] = int32(uint32(in[23+inpos])>>20) |
//		((in[24+inpos] & 32767) << (27 - 15))
//
//	out[29+outpos] = int32(uint32(in[24+inpos])>>15) |
//		((in[25+inpos] & 1023) << (27 - 10))
//
//	out[30+outpos] = int32(uint32(in[25+inpos])>>10) |
//		((in[26+inpos] & 31) << (27 - 5))
//
//	out[31+outpos] = int32(uint32(in[26+inpos]) >> 5)
//
//}
//
//func fastunpack28(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 268435455)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>28) |
//		((in[1+inpos] & 16777215) << (28 - 24))
//
//	out[2+outpos] = int32(uint32(in[1+inpos])>>24) |
//		((in[2+inpos] & 1048575) << (28 - 20))
//
//	out[3+outpos] = int32(uint32(in[2+inpos])>>20) |
//		((in[3+inpos] & 65535) << (28 - 16))
//
//	out[4+outpos] = int32(uint32(in[3+inpos])>>16) |
//		((in[4+inpos] & 4095) << (28 - 12))
//
//	out[5+outpos] = int32(uint32(in[4+inpos])>>12) |
//		((in[5+inpos] & 255) << (28 - 8))
//
//	out[6+outpos] = int32(uint32(in[5+inpos])>>8) |
//		((in[6+inpos] & 15) << (28 - 4))
//
//	out[7+outpos] = int32(uint32(in[6+inpos]) >> 4)
//
//	out[8+outpos] = (int32(uint32(in[7+inpos])>>0) & 268435455)
//
//	out[9+outpos] = int32(uint32(in[7+inpos])>>28) |
//		((in[8+inpos] & 16777215) << (28 - 24))
//
//	out[10+outpos] = int32(uint32(in[8+inpos])>>24) |
//		((in[9+inpos] & 1048575) << (28 - 20))
//
//	out[11+outpos] = int32(uint32(in[9+inpos])>>20) |
//		((in[10+inpos] & 65535) << (28 - 16))
//
//	out[12+outpos] = int32(uint32(in[10+inpos])>>16) |
//		((in[11+inpos] & 4095) << (28 - 12))
//
//	out[13+outpos] = int32(uint32(in[11+inpos])>>12) |
//		((in[12+inpos] & 255) << (28 - 8))
//
//	out[14+outpos] = int32(uint32(in[12+inpos])>>8) |
//		((in[13+inpos] & 15) << (28 - 4))
//
//	out[15+outpos] = int32(uint32(in[13+inpos]) >> 4)
//
//	out[16+outpos] = (int32(uint32(in[14+inpos])>>0) & 268435455)
//
//	out[17+outpos] = int32(uint32(in[14+inpos])>>28) |
//		((in[15+inpos] & 16777215) << (28 - 24))
//
//	out[18+outpos] = int32(uint32(in[15+inpos])>>24) |
//		((in[16+inpos] & 1048575) << (28 - 20))
//
//	out[19+outpos] = int32(uint32(in[16+inpos])>>20) |
//		((in[17+inpos] & 65535) << (28 - 16))
//
//	out[20+outpos] = int32(uint32(in[17+inpos])>>16) |
//		((in[18+inpos] & 4095) << (28 - 12))
//
//	out[21+outpos] = int32(uint32(in[18+inpos])>>12) |
//		((in[19+inpos] & 255) << (28 - 8))
//
//	out[22+outpos] = int32(uint32(in[19+inpos])>>8) |
//		((in[20+inpos] & 15) << (28 - 4))
//
//	out[23+outpos] = int32(uint32(in[20+inpos]) >> 4)
//
//	out[24+outpos] = (int32(uint32(in[21+inpos])>>0) & 268435455)
//
//	out[25+outpos] = int32(uint32(in[21+inpos])>>28) |
//		((in[22+inpos] & 16777215) << (28 - 24))
//
//	out[26+outpos] = int32(uint32(in[22+inpos])>>24) |
//		((in[23+inpos] & 1048575) << (28 - 20))
//
//	out[27+outpos] = int32(uint32(in[23+inpos])>>20) |
//		((in[24+inpos] & 65535) << (28 - 16))
//
//	out[28+outpos] = int32(uint32(in[24+inpos])>>16) |
//		((in[25+inpos] & 4095) << (28 - 12))
//
//	out[29+outpos] = int32(uint32(in[25+inpos])>>12) |
//		((in[26+inpos] & 255) << (28 - 8))
//
//	out[30+outpos] = int32(uint32(in[26+inpos])>>8) |
//		((in[27+inpos] & 15) << (28 - 4))
//
//	out[31+outpos] = int32(uint32(in[27+inpos]) >> 4)
//
//}
//
//func fastunpack29(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 536870911)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>29) |
//		((in[1+inpos] & 67108863) << (29 - 26))
//
//	out[2+outpos] = int32(uint32(in[1+inpos])>>26) |
//		((in[2+inpos] & 8388607) << (29 - 23))
//
//	out[3+outpos] = int32(uint32(in[2+inpos])>>23) |
//		((in[3+inpos] & 1048575) << (29 - 20))
//
//	out[4+outpos] = int32(uint32(in[3+inpos])>>20) |
//		((in[4+inpos] & 131071) << (29 - 17))
//
//	out[5+outpos] = int32(uint32(in[4+inpos])>>17) |
//		((in[5+inpos] & 16383) << (29 - 14))
//
//	out[6+outpos] = int32(uint32(in[5+inpos])>>14) |
//		((in[6+inpos] & 2047) << (29 - 11))
//
//	out[7+outpos] = int32(uint32(in[6+inpos])>>11) |
//		((in[7+inpos] & 255) << (29 - 8))
//
//	out[8+outpos] = int32(uint32(in[7+inpos])>>8) |
//		((in[8+inpos] & 31) << (29 - 5))
//
//	out[9+outpos] = int32(uint32(in[8+inpos])>>5) |
//		((in[9+inpos] & 3) << (29 - 2))
//
//	out[10+outpos] = (int32(uint32(in[9+inpos])>>2) & 536870911)
//
//	out[11+outpos] = int32(uint32(in[9+inpos])>>31) |
//		((in[10+inpos] & 268435455) << (29 - 28))
//
//	out[12+outpos] = int32(uint32(in[10+inpos])>>28) |
//		((in[11+inpos] & 33554431) << (29 - 25))
//
//	out[13+outpos] = int32(uint32(in[11+inpos])>>25) |
//		((in[12+inpos] & 4194303) << (29 - 22))
//
//	out[14+outpos] = int32(uint32(in[12+inpos])>>22) |
//		((in[13+inpos] & 524287) << (29 - 19))
//
//	out[15+outpos] = int32(uint32(in[13+inpos])>>19) |
//		((in[14+inpos] & 65535) << (29 - 16))
//
//	out[16+outpos] = int32(uint32(in[14+inpos])>>16) |
//		((in[15+inpos] & 8191) << (29 - 13))
//
//	out[17+outpos] = int32(uint32(in[15+inpos])>>13) |
//		((in[16+inpos] & 1023) << (29 - 10))
//
//	out[18+outpos] = int32(uint32(in[16+inpos])>>10) |
//		((in[17+inpos] & 127) << (29 - 7))
//
//	out[19+outpos] = int32(uint32(in[17+inpos])>>7) |
//		((in[18+inpos] & 15) << (29 - 4))
//
//	out[20+outpos] = int32(uint32(in[18+inpos])>>4) |
//		((in[19+inpos] & 1) << (29 - 1))
//
//	out[21+outpos] = (int32(uint32(in[19+inpos])>>1) & 536870911)
//
//	out[22+outpos] = int32(uint32(in[19+inpos])>>30) |
//		((in[20+inpos] & 134217727) << (29 - 27))
//
//	out[23+outpos] = int32(uint32(in[20+inpos])>>27) |
//		((in[21+inpos] & 16777215) << (29 - 24))
//
//	out[24+outpos] = int32(uint32(in[21+inpos])>>24) |
//		((in[22+inpos] & 2097151) << (29 - 21))
//
//	out[25+outpos] = int32(uint32(in[22+inpos])>>21) |
//		((in[23+inpos] & 262143) << (29 - 18))
//
//	out[26+outpos] = int32(uint32(in[23+inpos])>>18) |
//		((in[24+inpos] & 32767) << (29 - 15))
//
//	out[27+outpos] = int32(uint32(in[24+inpos])>>15) |
//		((in[25+inpos] & 4095) << (29 - 12))
//
//	out[28+outpos] = int32(uint32(in[25+inpos])>>12) |
//		((in[26+inpos] & 511) << (29 - 9))
//
//	out[29+outpos] = int32(uint32(in[26+inpos])>>9) |
//		((in[27+inpos] & 63) << (29 - 6))
//
//	out[30+outpos] = int32(uint32(in[27+inpos])>>6) |
//		((in[28+inpos] & 7) << (29 - 3))
//
//	out[31+outpos] = int32(uint32(in[28+inpos]) >> 3)
//
//}
//
//func fastunpack30(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 1073741823)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>30) |
//		((in[1+inpos] & 268435455) << (30 - 28))
//
//	out[2+outpos] = int32(uint32(in[1+inpos])>>28) |
//		((in[2+inpos] & 67108863) << (30 - 26))
//
//	out[3+outpos] = int32(uint32(in[2+inpos])>>26) |
//		((in[3+inpos] & 16777215) << (30 - 24))
//
//	out[4+outpos] = int32(uint32(in[3+inpos])>>24) |
//		((in[4+inpos] & 4194303) << (30 - 22))
//
//	out[5+outpos] = int32(uint32(in[4+inpos])>>22) |
//		((in[5+inpos] & 1048575) << (30 - 20))
//
//	out[6+outpos] = int32(uint32(in[5+inpos])>>20) |
//		((in[6+inpos] & 262143) << (30 - 18))
//
//	out[7+outpos] = int32(uint32(in[6+inpos])>>18) |
//		((in[7+inpos] & 65535) << (30 - 16))
//
//	out[8+outpos] = int32(uint32(in[7+inpos])>>16) |
//		((in[8+inpos] & 16383) << (30 - 14))
//
//	out[9+outpos] = int32(uint32(in[8+inpos])>>14) |
//		((in[9+inpos] & 4095) << (30 - 12))
//
//	out[10+outpos] = int32(uint32(in[9+inpos])>>12) |
//		((in[10+inpos] & 1023) << (30 - 10))
//
//	out[11+outpos] = int32(uint32(in[10+inpos])>>10) |
//		((in[11+inpos] & 255) << (30 - 8))
//
//	out[12+outpos] = int32(uint32(in[11+inpos])>>8) |
//		((in[12+inpos] & 63) << (30 - 6))
//
//	out[13+outpos] = int32(uint32(in[12+inpos])>>6) |
//		((in[13+inpos] & 15) << (30 - 4))
//
//	out[14+outpos] = int32(uint32(in[13+inpos])>>4) |
//		((in[14+inpos] & 3) << (30 - 2))
//
//	out[15+outpos] = int32(uint32(in[14+inpos]) >> 2)
//
//	out[16+outpos] = (int32(uint32(in[15+inpos])>>0) & 1073741823)
//
//	out[17+outpos] = int32(uint32(in[15+inpos])>>30) |
//		((in[16+inpos] & 268435455) << (30 - 28))
//
//	out[18+outpos] = int32(uint32(in[16+inpos])>>28) |
//		((in[17+inpos] & 67108863) << (30 - 26))
//
//	out[19+outpos] = int32(uint32(in[17+inpos])>>26) |
//		((in[18+inpos] & 16777215) << (30 - 24))
//
//	out[20+outpos] = int32(uint32(in[18+inpos])>>24) |
//		((in[19+inpos] & 4194303) << (30 - 22))
//
//	out[21+outpos] = int32(uint32(in[19+inpos])>>22) |
//		((in[20+inpos] & 1048575) << (30 - 20))
//
//	out[22+outpos] = int32(uint32(in[20+inpos])>>20) |
//		((in[21+inpos] & 262143) << (30 - 18))
//
//	out[23+outpos] = int32(uint32(in[21+inpos])>>18) |
//		((in[22+inpos] & 65535) << (30 - 16))
//
//	out[24+outpos] = int32(uint32(in[22+inpos])>>16) |
//		((in[23+inpos] & 16383) << (30 - 14))
//
//	out[25+outpos] = int32(uint32(in[23+inpos])>>14) |
//		((in[24+inpos] & 4095) << (30 - 12))
//
//	out[26+outpos] = int32(uint32(in[24+inpos])>>12) |
//		((in[25+inpos] & 1023) << (30 - 10))
//
//	out[27+outpos] = int32(uint32(in[25+inpos])>>10) |
//		((in[26+inpos] & 255) << (30 - 8))
//
//	out[28+outpos] = int32(uint32(in[26+inpos])>>8) |
//		((in[27+inpos] & 63) << (30 - 6))
//
//	out[29+outpos] = int32(uint32(in[27+inpos])>>6) |
//		((in[28+inpos] & 15) << (30 - 4))
//
//	out[30+outpos] = int32(uint32(in[28+inpos])>>4) |
//		((in[29+inpos] & 3) << (30 - 2))
//
//	out[31+outpos] = int32(uint32(in[29+inpos]) >> 2)
//
//}
//
//func fastunpack31(in []int32, inpos int, out []int32, outpos int) {
//	out[0+outpos] = (int32(uint32(in[0+inpos])>>0) & 2147483647)
//
//	out[1+outpos] = int32(uint32(in[0+inpos])>>31) |
//		((in[1+inpos] & 1073741823) << (31 - 30))
//
//	out[2+outpos] = int32(uint32(in[1+inpos])>>30) |
//		((in[2+inpos] & 536870911) << (31 - 29))
//
//	out[3+outpos] = int32(uint32(in[2+inpos])>>29) |
//		((in[3+inpos] & 268435455) << (31 - 28))
//
//	out[4+outpos] = int32(uint32(in[3+inpos])>>28) |
//		((in[4+inpos] & 134217727) << (31 - 27))
//
//	out[5+outpos] = int32(uint32(in[4+inpos])>>27) |
//		((in[5+inpos] & 67108863) << (31 - 26))
//
//	out[6+outpos] = int32(uint32(in[5+inpos])>>26) |
//		((in[6+inpos] & 33554431) << (31 - 25))
//
//	out[7+outpos] = int32(uint32(in[6+inpos])>>25) |
//		((in[7+inpos] & 16777215) << (31 - 24))
//
//	out[8+outpos] = int32(uint32(in[7+inpos])>>24) |
//		((in[8+inpos] & 8388607) << (31 - 23))
//
//	out[9+outpos] = int32(uint32(in[8+inpos])>>23) |
//		((in[9+inpos] & 4194303) << (31 - 22))
//
//	out[10+outpos] = int32(uint32(in[9+inpos])>>22) |
//		((in[10+inpos] & 2097151) << (31 - 21))
//
//	out[11+outpos] = int32(uint32(in[10+inpos])>>21) |
//		((in[11+inpos] & 1048575) << (31 - 20))
//
//	out[12+outpos] = int32(uint32(in[11+inpos])>>20) |
//		((in[12+inpos] & 524287) << (31 - 19))
//
//	out[13+outpos] = int32(uint32(in[12+inpos])>>19) |
//		((in[13+inpos] & 262143) << (31 - 18))
//
//	out[14+outpos] = int32(uint32(in[13+inpos])>>18) |
//		((in[14+inpos] & 131071) << (31 - 17))
//
//	out[15+outpos] = int32(uint32(in[14+inpos])>>17) |
//		((in[15+inpos] & 65535) << (31 - 16))
//
//	out[16+outpos] = int32(uint32(in[15+inpos])>>16) |
//		((in[16+inpos] & 32767) << (31 - 15))
//
//	out[17+outpos] = int32(uint32(in[16+inpos])>>15) |
//		((in[17+inpos] & 16383) << (31 - 14))
//
//	out[18+outpos] = int32(uint32(in[17+inpos])>>14) |
//		((in[18+inpos] & 8191) << (31 - 13))
//
//	out[19+outpos] = int32(uint32(in[18+inpos])>>13) |
//		((in[19+inpos] & 4095) << (31 - 12))
//
//	out[20+outpos] = int32(uint32(in[19+inpos])>>12) |
//		((in[20+inpos] & 2047) << (31 - 11))
//
//	out[21+outpos] = int32(uint32(in[20+inpos])>>11) |
//		((in[21+inpos] & 1023) << (31 - 10))
//
//	out[22+outpos] = int32(uint32(in[21+inpos])>>10) |
//		((in[22+inpos] & 511) << (31 - 9))
//
//	out[23+outpos] = int32(uint32(in[22+inpos])>>9) |
//		((in[23+inpos] & 255) << (31 - 8))
//
//	out[24+outpos] = int32(uint32(in[23+inpos])>>8) |
//		((in[24+inpos] & 127) << (31 - 7))
//
//	out[25+outpos] = int32(uint32(in[24+inpos])>>7) |
//		((in[25+inpos] & 63) << (31 - 6))
//
//	out[26+outpos] = int32(uint32(in[25+inpos])>>6) |
//		((in[26+inpos] & 31) << (31 - 5))
//
//	out[27+outpos] = int32(uint32(in[26+inpos])>>5) |
//		((in[27+inpos] & 15) << (31 - 4))
//
//	out[28+outpos] = int32(uint32(in[27+inpos])>>4) |
//		((in[28+inpos] & 7) << (31 - 3))
//
//	out[29+outpos] = int32(uint32(in[28+inpos])>>3) |
//		((in[29+inpos] & 3) << (31 - 2))
//
//	out[30+outpos] = int32(uint32(in[29+inpos])>>2) |
//		((in[30+inpos] & 1) << (31 - 1))
//
//	out[31+outpos] = int32(uint32(in[30+inpos]) >> 1)
//
//}
//
//func fastunpack32(in []int32, inpos int, out []int32, outpos int) {
//	copy(out[outpos:outpos+32], in[inpos:inpos+32])
//}
