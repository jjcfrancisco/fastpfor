pub fn fast_unpack(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize, bit: isize) {
    match bit {
        0 => fast_unpack0(output, outpos as usize),
        1 => fast_unpack1(input, inpos, output, outpos),
        2 => fast_unpack2(input, inpos, output, outpos),
        _ => panic!("Unsupported bit width"),
    }
}

fn fast_unpack0(output: &mut [i32], outpos: usize) {
    for i in outpos..outpos + 32 {
        output[i] = 0;
    }
}

fn fast_unpack1(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 1;
    output[1 + outpos] = (input[0 + inpos] >> 1) & 1;
    output[2 + outpos] = (input[0 + inpos] >> 2) & 1;
    output[3 + outpos] = (input[0 + inpos] >> 3) & 1;
    output[4 + outpos] = (input[0 + inpos] >> 4) & 1;
    output[5 + outpos] = (input[0 + inpos] >> 5) & 1;
    output[6 + outpos] = (input[0 + inpos] >> 6) & 1;
    output[7 + outpos] = (input[0 + inpos] >> 7) & 1;
    output[8 + outpos] = (input[0 + inpos] >> 8) & 1;
    output[9 + outpos] = (input[0 + inpos] >> 9) & 1;
    output[10 + outpos] = (input[0 + inpos] >> 10) & 1;
    output[11 + outpos] = (input[0 + inpos] >> 11) & 1;
    output[12 + outpos] = (input[0 + inpos] >> 12) & 1;
    output[13 + outpos] = (input[0 + inpos] >> 13) & 1;
    output[14 + outpos] = (input[0 + inpos] >> 14) & 1;
    output[15 + outpos] = (input[0 + inpos] >> 15) & 1;
    output[16 + outpos] = (input[0 + inpos] >> 16) & 1;
    output[17 + outpos] = (input[0 + inpos] >> 17) & 1;
    output[18 + outpos] = (input[0 + inpos] >> 18) & 1;
    output[19 + outpos] = (input[0 + inpos] >> 19) & 1;
    output[20 + outpos] = (input[0 + inpos] >> 20) & 1;
    output[21 + outpos] = (input[0 + inpos] >> 21) & 1;
    output[22 + outpos] = (input[0 + inpos] >> 22) & 1;
    output[23 + outpos] = (input[0 + inpos] >> 23) & 1;
    output[24 + outpos] = (input[0 + inpos] >> 24) & 1;
    output[25 + outpos] = (input[0 + inpos] >> 25) & 1;
    output[26 + outpos] = (input[0 + inpos] >> 26) & 1;
    output[27 + outpos] = (input[0 + inpos] >> 27) & 1;
    output[28 + outpos] = (input[0 + inpos] >> 28) & 1;
    output[29 + outpos] = (input[0 + inpos] >> 29) & 1;
    output[30 + outpos] = (input[0 + inpos] >> 30) & 1;
    output[31 + outpos] = input[0 + inpos] >> 31;
}

fn fast_unpack2(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 3;
    output[1 + outpos] = (input[0 + inpos] >> 2) & 3;
    output[2 + outpos] = (input[0 + inpos] >> 4) & 3;
    output[3 + outpos] = (input[0 + inpos] >> 6) & 3;
    output[4 + outpos] = (input[0 + inpos] >> 8) & 3;
    output[5 + outpos] = (input[0 + inpos] >> 10) & 3;
    output[6 + outpos] = (input[0 + inpos] >> 12) & 3;
    output[7 + outpos] = (input[0 + inpos] >> 14) & 3;
    output[8 + outpos] = (input[0 + inpos] >> 16) & 3;
    output[9 + outpos] = (input[0 + inpos] >> 18) & 3;
    output[10 + outpos] = (input[0 + inpos] >> 20) & 3;
    output[11 + outpos] = (input[0 + inpos] >> 22) & 3;
    output[12 + outpos] = (input[0 + inpos] >> 24) & 3;
    output[13 + outpos] = (input[0 + inpos] >> 26) & 3;
    output[14 + outpos] = (input[0 + inpos] >> 28) & 3;
    output[15 + outpos] = input[0 + inpos] >> 30;
    output[16 + outpos] = (input[1 + inpos] >> 0) & 3;
    output[17 + outpos] = (input[1 + inpos] >> 2) & 3;
    output[18 + outpos] = (input[1 + inpos] >> 4) & 3;
    output[19 + outpos] = (input[1 + inpos] >> 6) & 3;
    output[20 + outpos] = (input[1 + inpos] >> 8) & 3;
    output[21 + outpos] = (input[1 + inpos] >> 10) & 3;
    output[22 + outpos] = (input[1 + inpos] >> 12) & 3;
    output[23 + outpos] = (input[1 + inpos] >> 14) & 3;
    output[24 + outpos] = (input[1 + inpos] >> 16) & 3;
    output[25 + outpos] = (input[1 + inpos] >> 18) & 3;
    output[26 + outpos] = (input[1 + inpos] >> 20) & 3;
    output[27 + outpos] = (input[1 + inpos] >> 22) & 3;
    output[28 + outpos] = (input[1 + inpos] >> 24) & 3;
    output[29 + outpos] = (input[1 + inpos] >> 26) & 3;
    output[30 + outpos] = (input[1 + inpos] >> 28) & 3;
    output[31 + outpos] = input[1 + inpos] >> 30;
}

fn fast_unpack3(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 7;
    output[1 + outpos] = (input[0 + inpos] >> 3) & 7;
    output[2 + outpos] = (input[0 + inpos] >> 6) & 7;
    output[3 + outpos] = (input[0 + inpos] >> 9) & 7;
    output[4 + outpos] = (input[0 + inpos] >> 12) & 7;
    output[5 + outpos] = (input[0 + inpos] >> 15) & 7;
    output[6 + outpos] = (input[0 + inpos] >> 18) & 7;
    output[7 + outpos] = (input[0 + inpos] >> 21) & 7;
    output[8 + outpos] = (input[0 + inpos] >> 24) & 7;
    output[9 + outpos] = (input[0 + inpos] >> 27) & 7;
    output[10 + outpos] = (input[0 + inpos] >> 30) | (input[1 + inpos] & 1) << (3 - 1);
    output[11 + outpos] = (input[1 + inpos] >> 1) & 7;
    output[12 + outpos] = (input[1 + inpos] >> 4) & 7;
    output[13 + outpos] = (input[1 + inpos] >> 7) & 7;
    output[14 + outpos] = (input[1 + inpos] >> 10) & 7;
    output[15 + outpos] = (input[1 + inpos] >> 13) & 7;
    output[16 + outpos] = (input[1 + inpos] >> 16) & 7;
    output[17 + outpos] = (input[1 + inpos] >> 19) & 7;
    output[18 + outpos] = (input[1 + inpos] >> 22) & 7;
    output[19 + outpos] = (input[1 + inpos] >> 25) & 7;
    output[20 + outpos] = (input[1 + inpos] >> 28) & 7;
    output[21 + outpos] = (input[1 + inpos] >> 31) | (input[2 + inpos] & 3) << (3 - 2);
    output[22 + outpos] = (input[2 + inpos] >> 2) & 7;
    output[23 + outpos] = (input[2 + inpos] >> 5) & 7;
    output[24 + outpos] = (input[2 + inpos] >> 8) & 7;
    output[25 + outpos] = (input[2 + inpos] >> 11) & 7;
    output[26 + outpos] = (input[2 + inpos] >> 14) & 7;
    output[27 + outpos] = (input[2 + inpos] >> 17) & 7;
    output[28 + outpos] = (input[2 + inpos] >> 20) & 7;
    output[29 + outpos] = (input[2 + inpos] >> 23) & 7;
    output[30 + outpos] = (input[2 + inpos] >> 26) & 7;
    output[31 + outpos] = input[2 + inpos] >> 29;
}

fn fast_unpack4(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 15;
    output[1 + outpos] = (input[0 + inpos] >> 4) & 15;
    output[2 + outpos] = (input[0 + inpos] >> 8) & 15;
    output[3 + outpos] = (input[0 + inpos] >> 12) & 15;
    output[4 + outpos] = (input[0 + inpos] >> 16) & 15;
    output[5 + outpos] = (input[0 + inpos] >> 20) & 15;
    output[6 + outpos] = (input[0 + inpos] >> 24) & 15;
    output[7 + outpos] = input[0 + inpos] >> 28;
    output[8 + outpos] = (input[1 + inpos] >> 0) & 15;
    output[9 + outpos] = (input[1 + inpos] >> 4) & 15;
    output[10 + outpos] = (input[1 + inpos] >> 8) & 15;
    output[11 + outpos] = (input[1 + inpos] >> 12) & 15;
    output[12 + outpos] = (input[1 + inpos] >> 16) & 15;
    output[13 + outpos] = (input[1 + inpos] >> 20) & 15;
    output[14 + outpos] = (input[1 + inpos] >> 24) & 15;
    output[15 + outpos] = input[1 + inpos] >> 28;
    output[16 + outpos] = (input[2 + inpos] >> 0) & 15;
    output[17 + outpos] = (input[2 + inpos] >> 4) & 15;
    output[18 + outpos] = (input[2 + inpos] >> 8) & 15;
    output[19 + outpos] = (input[2 + inpos] >> 12) & 15;
    output[20 + outpos] = (input[2 + inpos] >> 16) & 15;
    output[21 + outpos] = (input[2 + inpos] >> 20) & 15;
    output[22 + outpos] = (input[2 + inpos] >> 24) & 15;
    output[23 + outpos] = input[2 + inpos] >> 28;
    output[24 + outpos] = (input[3 + inpos] >> 0) & 15;
    output[25 + outpos] = (input[3 + inpos] >> 4) & 15;
    output[26 + outpos] = (input[3 + inpos] >> 8) & 15;
    output[27 + outpos] = (input[3 + inpos] >> 12) & 15;
    output[28 + outpos] = (input[3 + inpos] >> 16) & 15;
    output[29 + outpos] = (input[3 + inpos] >> 20) & 15;
    output[30 + outpos] = (input[3 + inpos] >> 24) & 15;
    output[31 + outpos] = input[3 + inpos] >> 28;
}

fn fast_unpack5(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 31;
    output[1 + outpos] = (input[0 + inpos] >> 5) & 31;
    output[2 + outpos] = (input[0 + inpos] >> 10) & 31;
    output[3 + outpos] = (input[0 + inpos] >> 15) & 31;
    output[4 + outpos] = (input[0 + inpos] >> 20) & 31;
    output[5 + outpos] = (input[0 + inpos] >> 25) & 31;
    output[6 + outpos] = (input[0 + inpos] >> 30) | ((input[1 + inpos] & 7) << (5 - 3));
    output[7 + outpos] = (input[1 + inpos] >> 3) & 31;
    output[8 + outpos] = (input[1 + inpos] >> 8) & 31;
    output[9 + outpos] = (input[1 + inpos] >> 13) & 31;
    output[10 + outpos] = (input[1 + inpos] >> 18) & 31;
    output[11 + outpos] = (input[1 + inpos] >> 23) & 31;
    output[12 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 1) << (5 - 1));
    output[13 + outpos] = (input[2 + inpos] >> 1) & 31;
    output[14 + outpos] = (input[2 + inpos] >> 6) & 31;
    output[15 + outpos] = (input[2 + inpos] >> 11) & 31;
    output[16 + outpos] = (input[2 + inpos] >> 16) & 31;
    output[17 + outpos] = (input[2 + inpos] >> 21) & 31;
    output[18 + outpos] = (input[2 + inpos] >> 26) & 31;
    output[19 + outpos] = (input[2 + inpos] >> 31) | ((input[3 + inpos] & 15) << (5 - 4));
    output[20 + outpos] = (input[3 + inpos] >> 4) & 31;
    output[21 + outpos] = (input[3 + inpos] >> 9) & 31;
    output[22 + outpos] = (input[3 + inpos] >> 14) & 31;
    output[23 + outpos] = (input[3 + inpos] >> 19) & 31;
    output[24 + outpos] = (input[3 + inpos] >> 24) & 31;
    output[25 + outpos] = (input[3 + inpos] >> 29) | ((input[4 + inpos] & 3) << (5 - 2));
    output[26 + outpos] = (input[4 + inpos] >> 2) & 31;
    output[27 + outpos] = (input[4 + inpos] >> 7) & 31;
    output[28 + outpos] = (input[4 + inpos] >> 12) & 31;
    output[29 + outpos] = (input[4 + inpos] >> 17) & 31;
    output[30 + outpos] = (input[4 + inpos] >> 22) & 31;
    output[31 + outpos] = input[4 + inpos] >> 27;
}

fn fast_unpack6(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 63;
    output[1 + outpos] = (input[0 + inpos] >> 6) & 63;
    output[2 + outpos] = (input[0 + inpos] >> 12) & 63;
    output[3 + outpos] = (input[0 + inpos] >> 18) & 63;
    output[4 + outpos] = (input[0 + inpos] >> 24) & 63;
    output[5 + outpos] = (input[0 + inpos] >> 30) | ((input[1 + inpos] & 15) << (6 - 4));
    output[6 + outpos] = (input[1 + inpos] >> 4) & 63;
    output[7 + outpos] = (input[1 + inpos] >> 10) & 63;
    output[8 + outpos] = (input[1 + inpos] >> 16) & 63;
    output[9 + outpos] = (input[1 + inpos] >> 22) & 63;
    output[10 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 3) << (6 - 2));
    output[11 + outpos] = (input[2 + inpos] >> 2) & 63;
    output[12 + outpos] = (input[2 + inpos] >> 8) & 63;
    output[13 + outpos] = (input[2 + inpos] >> 14) & 63;
    output[14 + outpos] = (input[2 + inpos] >> 20) & 63;
    output[15 + outpos] = input[2 + inpos] >> 26;
    output[16 + outpos] = (input[3 + inpos] >> 0) & 63;
    output[17 + outpos] = (input[3 + inpos] >> 6) & 63;
    output[18 + outpos] = (input[3 + inpos] >> 12) & 63;
    output[19 + outpos] = (input[3 + inpos] >> 18) & 63;
    output[20 + outpos] = (input[3 + inpos] >> 24) & 63;
    output[21 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 15) << (6 - 4));
    output[22 + outpos] = (input[4 + inpos] >> 4) & 63;
    output[23 + outpos] = (input[4 + inpos] >> 10) & 63;
    output[24 + outpos] = (input[4 + inpos] >> 16) & 63;
    output[25 + outpos] = (input[4 + inpos] >> 22) & 63;
    output[26 + outpos] = (input[4 + inpos] >> 28) | ((input[5 + inpos] & 3) << (6 - 2));
    output[27 + outpos] = (input[5 + inpos] >> 2) & 63;
    output[28 + outpos] = (input[5 + inpos] >> 8) & 63;
    output[29 + outpos] = (input[5 + inpos] >> 14) & 63;
    output[30 + outpos] = (input[5 + inpos] >> 20) & 63;
    output[31 + outpos] = input[5 + inpos] >> 26;
}

fn fast_unpack7(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 127;
    output[1 + outpos] = (input[0 + inpos] >> 7) & 127;
    output[2 + outpos] = (input[0 + inpos] >> 14) & 127;
    output[3 + outpos] = (input[0 + inpos] >> 21) & 127;
    output[4 + outpos] = (input[0 + inpos] >> 28) | ((input[1 + inpos] & 7) << (7 - 3));
    output[5 + outpos] = (input[1 + inpos] >> 3) & 127;
    output[6 + outpos] = (input[1 + inpos] >> 10) & 127;
    output[7 + outpos] = (input[1 + inpos] >> 17) & 127;
    output[8 + outpos] = (input[1 + inpos] >> 24) & 127;
    output[9 + outpos] = (input[1 + inpos] >> 31) | ((input[2 + inpos] & 63) << (7 - 6));
    output[10 + outpos] = (input[2 + inpos] >> 6) & 127;
    output[11 + outpos] = (input[2 + inpos] >> 13) & 127;
    output[12 + outpos] = (input[2 + inpos] >> 20) & 127;
    output[13 + outpos] = (input[2 + inpos] >> 27) | ((input[3 + inpos] & 3) << (7 - 2));
    output[14 + outpos] = (input[3 + inpos] >> 2) & 127;
    output[15 + outpos] = (input[3 + inpos] >> 9) & 127;
    output[16 + outpos] = (input[3 + inpos] >> 16) & 127;
    output[17 + outpos] = (input[3 + inpos] >> 23) & 127;
    output[18 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 31) << (7 - 5));
    output[19 + outpos] = (input[4 + inpos] >> 5) & 127;
    output[20 + outpos] = (input[4 + inpos] >> 12) & 127;
    output[21 + outpos] = (input[4 + inpos] >> 19) & 127;
    output[22 + outpos] = (input[4 + inpos] >> 26) | ((input[5 + inpos] & 1) << (7 - 1));
    output[23 + outpos] = (input[5 + inpos] >> 1) & 127;
    output[24 + outpos] = (input[5 + inpos] >> 8) & 127;
    output[25 + outpos] = (input[5 + inpos] >> 15) & 127;
    output[26 + outpos] = (input[5 + inpos] >> 22) & 127;
    output[27 + outpos] = (input[5 + inpos] >> 29) | ((input[6 + inpos] & 15) << (7 - 4));
    output[28 + outpos] = (input[6 + inpos] >> 4) & 127;
    output[29 + outpos] = (input[6 + inpos] >> 11) & 127;
    output[30 + outpos] = (input[6 + inpos] >> 18) & 127;
    output[31 + outpos] = input[6 + inpos] >> 25;
}

fn fast_unpack8(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 255;
    output[1 + outpos] = (input[0 + inpos] >> 8) & 255;
    output[2 + outpos] = (input[0 + inpos] >> 16) & 255;
    output[3 + outpos] = input[0 + inpos] >> 24;
    output[4 + outpos] = (input[1 + inpos] >> 0) & 255;
    output[5 + outpos] = (input[1 + inpos] >> 8) & 255;
    output[6 + outpos] = (input[1 + inpos] >> 16) & 255;
    output[7 + outpos] = input[1 + inpos] >> 24;
    output[8 + outpos] = (input[2 + inpos] >> 0) & 255;
    output[9 + outpos] = (input[2 + inpos] >> 8) & 255;
    output[10 + outpos] = (input[2 + inpos] >> 16) & 255;
    output[11 + outpos] = input[2 + inpos] >> 24;
    output[12 + outpos] = (input[3 + inpos] >> 0) & 255;
    output[13 + outpos] = (input[3 + inpos] >> 8) & 255;
    output[14 + outpos] = (input[3 + inpos] >> 16) & 255;
    output[15 + outpos] = input[3 + inpos] >> 24;
    output[16 + outpos] = (input[4 + inpos] >> 0) & 255;
    output[17 + outpos] = (input[4 + inpos] >> 8) & 255;
    output[18 + outpos] = (input[4 + inpos] >> 16) & 255;
    output[19 + outpos] = input[4 + inpos] >> 24;
    output[20 + outpos] = (input[5 + inpos] >> 0) & 255;
    output[21 + outpos] = (input[5 + inpos] >> 8) & 255;
    output[22 + outpos] = (input[5 + inpos] >> 16) & 255;
    output[23 + outpos] = input[5 + inpos] >> 24;
    output[24 + outpos] = (input[6 + inpos] >> 0) & 255;
    output[25 + outpos] = (input[6 + inpos] >> 8) & 255;
    output[26 + outpos] = (input[6 + inpos] >> 16) & 255;
    output[27 + outpos] = input[6 + inpos] >> 24;
    output[28 + outpos] = (input[7 + inpos] >> 0) & 255;
    output[29 + outpos] = (input[7 + inpos] >> 8) & 255;
    output[30 + outpos] = (input[7 + inpos] >> 16) & 255;
    output[31 + outpos] = input[7 + inpos] >> 24;
}

fn fast_unpack9(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 511;
    output[1 + outpos] = (input[0 + inpos] >> 9) & 511;
    output[2 + outpos] = (input[0 + inpos] >> 18) & 511;
    output[3 + outpos] = (input[0 + inpos] >> 27) | ((input[1 + inpos] & 15) << (9 - 4));
    output[4 + outpos] = (input[1 + inpos] >> 4) & 511;
    output[5 + outpos] = (input[1 + inpos] >> 13) & 511;
    output[6 + outpos] = (input[1 + inpos] >> 22) & 511;
    output[7 + outpos] = (input[1 + inpos] >> 31) | ((input[2 + inpos] & 255) << (9 - 8));
    output[8 + outpos] = (input[2 + inpos] >> 8) & 511;
    output[9 + outpos] = (input[2 + inpos] >> 17) & 511;
    output[10 + outpos] = (input[2 + inpos] >> 26) | ((input[3 + inpos] & 7) << (9 - 3));
    output[11 + outpos] = (input[3 + inpos] >> 3) & 511;
    output[12 + outpos] = (input[3 + inpos] >> 12) & 511;
    output[13 + outpos] = (input[3 + inpos] >> 21) & 511;
    output[14 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 127) << (9 - 7));
    output[15 + outpos] = (input[4 + inpos] >> 7) & 511;
    output[16 + outpos] = (input[4 + inpos] >> 16) & 511;
    output[17 + outpos] = (input[4 + inpos] >> 25) | ((input[5 + inpos] & 3) << (9 - 2));
    output[18 + outpos] = (input[5 + inpos] >> 2) & 511;
    output[19 + outpos] = (input[5 + inpos] >> 11) & 511;
    output[20 + outpos] = (input[5 + inpos] >> 20) & 511;
    output[21 + outpos] = (input[5 + inpos] >> 29) | ((input[6 + inpos] & 63) << (9 - 6));
    output[22 + outpos] = (input[6 + inpos] >> 6) & 511;
    output[23 + outpos] = (input[6 + inpos] >> 15) & 511;
    output[24 + outpos] = (input[6 + inpos] >> 24) | ((input[7 + inpos] & 1) << (9 - 1));
    output[25 + outpos] = (input[7 + inpos] >> 1) & 511;
    output[26 + outpos] = (input[7 + inpos] >> 10) & 511;
    output[27 + outpos] = (input[7 + inpos] >> 19) & 511;
    output[28 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 31) << (9 - 5));
    output[29 + outpos] = (input[8 + inpos] >> 5) & 511;
    output[30 + outpos] = (input[8 + inpos] >> 14) & 511;
    output[31 + outpos] = input[8 + inpos] >> 23;
}

fn fast_unpack10(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 1023;
    output[1 + outpos] = (input[0 + inpos] >> 10) & 1023;
    output[2 + outpos] = (input[0 + inpos] >> 20) & 1023;
    output[3 + outpos] = (input[0 + inpos] >> 30) | ((input[1 + inpos] & 255) << (10 - 8));
    output[4 + outpos] = (input[1 + inpos] >> 8) & 1023;
    output[5 + outpos] = (input[1 + inpos] >> 18) & 1023;
    output[6 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 63) << (10 - 6));
    output[7 + outpos] = (input[2 + inpos] >> 6) & 1023;
    output[8 + outpos] = (input[2 + inpos] >> 16) & 1023;
    output[9 + outpos] = (input[2 + inpos] >> 26) | ((input[3 + inpos] & 15) << (10 - 4));
    output[10 + outpos] = (input[3 + inpos] >> 4) & 1023;
    output[11 + outpos] = (input[3 + inpos] >> 14) & 1023;
    output[12 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 3) << (10 - 2));
    output[13 + outpos] = (input[4 + inpos] >> 2) & 1023;
    output[14 + outpos] = (input[4 + inpos] >> 12) & 1023;
    output[15 + outpos] = input[4 + inpos] >> 22;
    output[16 + outpos] = (input[5 + inpos] >> 0) & 1023;
    output[17 + outpos] = (input[5 + inpos] >> 10) & 1023;
    output[18 + outpos] = (input[5 + inpos] >> 20) & 1023;
    output[19 + outpos] = (input[5 + inpos] >> 30) | ((input[6 + inpos] & 255) << (10 - 8));
    output[20 + outpos] = (input[6 + inpos] >> 8) & 1023;
    output[21 + outpos] = (input[6 + inpos] >> 18) & 1023;
    output[22 + outpos] = (input[6 + inpos] >> 28) | ((input[7 + inpos] & 63) << (10 - 6));
    output[23 + outpos] = (input[7 + inpos] >> 6) & 1023;
    output[24 + outpos] = (input[7 + inpos] >> 16) & 1023;
    output[25 + outpos] = (input[7 + inpos] >> 26) | ((input[8 + inpos] & 15) << (10 - 4));
    output[26 + outpos] = (input[8 + inpos] >> 4) & 1023;
    output[27 + outpos] = (input[8 + inpos] >> 14) & 1023;
    output[28 + outpos] = (input[8 + inpos] >> 24) | ((input[9 + inpos] & 3) << (10 - 2));
    output[29 + outpos] = (input[9 + inpos] >> 2) & 1023;
    output[30 + outpos] = (input[9 + inpos] >> 12) & 1023;
    output[31 + outpos] = input[9 + inpos] >> 22;
}

fn fast_unpack11(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 2047;
    output[1 + outpos] = (input[0 + inpos] >> 11) & 2047;
    output[2 + outpos] = (input[0 + inpos] >> 22) | ((input[1 + inpos] & 1) << (11 - 1));
    output[3 + outpos] = (input[1 + inpos] >> 1) & 2047;
    output[4 + outpos] = (input[1 + inpos] >> 12) & 2047;
    output[5 + outpos] = (input[1 + inpos] >> 23) | ((input[2 + inpos] & 3) << (11 - 2));
    output[6 + outpos] = (input[2 + inpos] >> 2) & 2047;
    output[7 + outpos] = (input[2 + inpos] >> 13) & 2047;
    output[8 + outpos] = (input[2 + inpos] >> 24) | ((input[3 + inpos] & 7) << (11 - 3));
    output[9 + outpos] = (input[3 + inpos] >> 3) & 2047;
    output[10 + outpos] = (input[3 + inpos] >> 14) & 2047;
    output[11 + outpos] = (input[3 + inpos] >> 25) | ((input[4 + inpos] & 15) << (11 - 4));
    output[12 + outpos] = (input[4 + inpos] >> 4) & 2047;
    output[13 + outpos] = (input[4 + inpos] >> 15) & 2047;
    output[14 + outpos] = (input[4 + inpos] >> 26) | ((input[5 + inpos] & 31) << (11 - 5));
    output[15 + outpos] = (input[5 + inpos] >> 5) & 2047;
    output[16 + outpos] = (input[5 + inpos] >> 16) & 2047;
    output[17 + outpos] = (input[5 + inpos] >> 27) | ((input[6 + inpos] & 63) << (11 - 6));
    output[18 + outpos] = (input[6 + inpos] >> 6) & 2047;
    output[19 + outpos] = (input[6 + inpos] >> 17) & 2047;
    output[20 + outpos] = (input[6 + inpos] >> 28) | ((input[7 + inpos] & 127) << (11 - 7));
    output[21 + outpos] = (input[7 + inpos] >> 7) & 2047;
    output[22 + outpos] = (input[7 + inpos] >> 18) & 2047;
    output[23 + outpos] = (input[7 + inpos] >> 29) | ((input[8 + inpos] & 255) << (11 - 8));
    output[24 + outpos] = (input[8 + inpos] >> 8) & 2047;
    output[25 + outpos] = (input[8 + inpos] >> 19) & 2047;
    output[26 + outpos] = (input[8 + inpos] >> 30) | ((input[9 + inpos] & 511) << (11 - 9));
    output[27 + outpos] = (input[9 + inpos] >> 9) & 2047;
    output[28 + outpos] = (input[9 + inpos] >> 20) & 2047;
    output[29 + outpos] = (input[9 + inpos] >> 31) | ((input[10 + inpos] & 1023) << (11 - 10));
    output[30 + outpos] = (input[10 + inpos] >> 10) & 2047;
    output[31 + outpos] = input[10 + inpos] >> 21;
}

fn fast_unpack12(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 4095;
    output[1 + outpos] = (input[0 + inpos] >> 12) & 4095;
    output[2 + outpos] = (input[0 + inpos] >> 24) | ((input[1 + inpos] & 15) << (12 - 4));
    output[3 + outpos] = (input[1 + inpos] >> 4) & 4095;
    output[4 + outpos] = (input[1 + inpos] >> 16) & 4095;
    output[5 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 255) << (12 - 8));
    output[6 + outpos] = (input[2 + inpos] >> 8) & 4095;
    output[7 + outpos] = input[2 + inpos] >> 20;
    output[8 + outpos] = (input[3 + inpos] >> 0) & 4095;
    output[9 + outpos] = (input[3 + inpos] >> 12) & 4095;
    output[10 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 15) << (12 - 4));
    output[11 + outpos] = (input[4 + inpos] >> 4) & 4095;
    output[12 + outpos] = (input[4 + inpos] >> 16) & 4095;
    output[13 + outpos] = (input[4 + inpos] >> 28) | ((input[5 + inpos] & 255) << (12 - 8));
    output[14 + outpos] = (input[5 + inpos] >> 8) & 4095;
    output[15 + outpos] = input[5 + inpos] >> 20;
    output[16 + outpos] = (input[6 + inpos] >> 0) & 4095;
    output[17 + outpos] = (input[6 + inpos] >> 12) & 4095;
    output[18 + outpos] = (input[6 + inpos] >> 24) | ((input[7 + inpos] & 15) << (12 - 4));
    output[19 + outpos] = (input[7 + inpos] >> 4) & 4095;
    output[20 + outpos] = (input[7 + inpos] >> 16) & 4095;
    output[21 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 255) << (12 - 8));
    output[22 + outpos] = (input[8 + inpos] >> 8) & 4095;
    output[23 + outpos] = input[8 + inpos] >> 20;
    output[24 + outpos] = (input[9 + inpos] >> 0) & 4095;
    output[25 + outpos] = (input[9 + inpos] >> 12) & 4095;
    output[26 + outpos] = (input[9 + inpos] >> 24) | ((input[10 + inpos] & 15) << (12 - 4));
    output[27 + outpos] = (input[10 + inpos] >> 4) & 4095;
    output[28 + outpos] = (input[10 + inpos] >> 16) & 4095;
    output[29 + outpos] = (input[10 + inpos] >> 28) | ((input[11 + inpos] & 255) << (12 - 8));
    output[30 + outpos] = (input[11 + inpos] >> 8) & 4095;
    output[31 + outpos] = input[11 + inpos] >> 20;
}

fn fast_unpack13(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 8191;
    output[1 + outpos] = (input[0 + inpos] >> 13) & 8191;
    output[2 + outpos] = (input[0 + inpos] >> 26) | ((input[1 + inpos] & 127) << (13 - 7));
    output[3 + outpos] = (input[1 + inpos] >> 7) & 8191;
    output[4 + outpos] = (input[1 + inpos] >> 20) | ((input[2 + inpos] & 1) << (13 - 1));
    output[5 + outpos] = (input[2 + inpos] >> 1) & 8191;
    output[6 + outpos] = (input[2 + inpos] >> 14) & 8191;
    output[7 + outpos] = (input[2 + inpos] >> 27) | ((input[3 + inpos] & 255) << (13 - 8));
    output[8 + outpos] = (input[3 + inpos] >> 8) & 8191;
    output[9 + outpos] = (input[3 + inpos] >> 21) | ((input[4 + inpos] & 3) << (13 - 2));
    output[10 + outpos] = (input[4 + inpos] >> 2) & 8191;
    output[11 + outpos] = (input[4 + inpos] >> 15) & 8191;
    output[12 + outpos] = (input[4 + inpos] >> 28) | ((input[5 + inpos] & 511) << (13 - 9));
    output[13 + outpos] = (input[5 + inpos] >> 9) & 8191;
    output[14 + outpos] = (input[5 + inpos] >> 22) | ((input[6 + inpos] & 7) << (13 - 3));
    output[15 + outpos] = (input[6 + inpos] >> 3) & 8191;
    output[16 + outpos] = (input[6 + inpos] >> 16) & 8191;
    output[17 + outpos] = (input[6 + inpos] >> 29) | ((input[7 + inpos] & 1023) << (13 - 10));
    output[18 + outpos] = (input[7 + inpos] >> 10) & 8191;
    output[19 + outpos] = (input[7 + inpos] >> 23) | ((input[8 + inpos] & 15) << (13 - 4));
    output[20 + outpos] = (input[8 + inpos] >> 4) & 8191;
    output[21 + outpos] = (input[8 + inpos] >> 17) & 8191;
    output[22 + outpos] = (input[8 + inpos] >> 30) | ((input[9 + inpos] & 2047) << (13 - 11));
    output[23 + outpos] = (input[9 + inpos] >> 11) & 8191;
    output[24 + outpos] = (input[9 + inpos] >> 24) | ((input[10 + inpos] & 31) << (13 - 5));
    output[25 + outpos] = (input[10 + inpos] >> 5) & 8191;
    output[26 + outpos] = (input[10 + inpos] >> 18) & 8191;
    output[27 + outpos] = (input[10 + inpos] >> 31) | ((input[11 + inpos] & 4095) << (13 - 12));
    output[28 + outpos] = (input[11 + inpos] >> 12) & 8191;
    output[29 + outpos] = (input[11 + inpos] >> 25) | ((input[12 + inpos] & 63) << (13 - 6));
    output[30 + outpos] = (input[12 + inpos] >> 6) & 8191;
    output[31 + outpos] = input[12 + inpos] >> 19;
}

fn fast_unpack14(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 16383;
    output[1 + outpos] = (input[0 + inpos] >> 14) & 16383;
    output[2 + outpos] = (input[0 + inpos] >> 28) | ((input[1 + inpos] & 1023) << (14 - 10));
    output[3 + outpos] = (input[1 + inpos] >> 10) & 16383;
    output[4 + outpos] = (input[1 + inpos] >> 24) | ((input[2 + inpos] & 63) << (14 - 6));
    output[5 + outpos] = (input[2 + inpos] >> 6) & 16383;
    output[6 + outpos] = (input[2 + inpos] >> 20) | ((input[3 + inpos] & 3) << (14 - 2));
    output[7 + outpos] = (input[3 + inpos] >> 2) & 16383;
    output[8 + outpos] = (input[3 + inpos] >> 16) & 16383;
    output[9 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 4095) << (14 - 12));
    output[10 + outpos] = (input[4 + inpos] >> 12) & 16383;
    output[11 + outpos] = (input[4 + inpos] >> 26) | ((input[5 + inpos] & 255) << (14 - 8));
    output[12 + outpos] = (input[5 + inpos] >> 8) & 16383;
    output[13 + outpos] = (input[5 + inpos] >> 22) | ((input[6 + inpos] & 15) << (14 - 4));
    output[14 + outpos] = (input[6 + inpos] >> 4) & 16383;
    output[15 + outpos] = input[6 + inpos] >> 18;
    output[16 + outpos] = (input[7 + inpos] >> 0) & 16383;
    output[17 + outpos] = (input[7 + inpos] >> 14) & 16383;
    output[18 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 1023) << (14 - 10));
    output[19 + outpos] = (input[8 + inpos] >> 10) & 16383;
    output[20 + outpos] = (input[8 + inpos] >> 24) | ((input[9 + inpos] & 63) << (14 - 6));
    output[21 + outpos] = (input[9 + inpos] >> 6) & 16383;
    output[22 + outpos] = (input[9 + inpos] >> 20) | ((input[10 + inpos] & 3) << (14 - 2));
    output[23 + outpos] = (input[10 + inpos] >> 2) & 16383;
    output[24 + outpos] = (input[10 + inpos] >> 16) & 16383;
    output[25 + outpos] = (input[10 + inpos] >> 30) | ((input[11 + inpos] & 4095) << (14 - 12));
    output[26 + outpos] = (input[11 + inpos] >> 12) & 16383;
    output[27 + outpos] = (input[11 + inpos] >> 26) | ((input[12 + inpos] & 255) << (14 - 8));
    output[28 + outpos] = (input[12 + inpos] >> 8) & 16383;
    output[29 + outpos] = (input[12 + inpos] >> 22) | ((input[13 + inpos] & 15) << (14 - 4));
    output[30 + outpos] = (input[13 + inpos] >> 4) & 16383;
    output[31 + outpos] = input[13 + inpos] >> 18;
}

fn fast_unpack15(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 32767;
    output[1 + outpos] = (input[0 + inpos] >> 15) & 32767;
    output[2 + outpos] = (input[0 + inpos] >> 30) | ((input[1 + inpos] & 8191) << (15 - 13));
    output[3 + outpos] = (input[1 + inpos] >> 13) & 32767;
    output[4 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 2047) << (15 - 11));
    output[5 + outpos] = (input[2 + inpos] >> 11) & 32767;
    output[6 + outpos] = (input[2 + inpos] >> 26) | ((input[3 + inpos] & 511) << (15 - 9));
    output[7 + outpos] = (input[3 + inpos] >> 9) & 32767;
    output[8 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 127) << (15 - 7));
    output[9 + outpos] = (input[4 + inpos] >> 7) & 32767;
    output[10 + outpos] = (input[4 + inpos] >> 22) | ((input[5 + inpos] & 31) << (15 - 5));
    output[11 + outpos] = (input[5 + inpos] >> 5) & 32767;
    output[12 + outpos] = (input[5 + inpos] >> 20) | ((input[6 + inpos] & 7) << (15 - 3));
    output[13 + outpos] = (input[6 + inpos] >> 3) & 32767;
    output[14 + outpos] = (input[6 + inpos] >> 18) | ((input[7 + inpos] & 1) << (15 - 1));
    output[15 + outpos] = (input[7 + inpos] >> 1) & 32767;
    output[16 + outpos] = (input[7 + inpos] >> 16) & 32767;
    output[17 + outpos] = (input[7 + inpos] >> 31) | ((input[8 + inpos] & 16383) << (15 - 14));
    output[18 + outpos] = (input[8 + inpos] >> 14) & 32767;
    output[19 + outpos] = (input[8 + inpos] >> 29) | ((input[9 + inpos] & 4095) << (15 - 12));
    output[20 + outpos] = (input[9 + inpos] >> 12) & 32767;
    output[21 + outpos] = (input[9 + inpos] >> 27) | ((input[10 + inpos] & 1023) << (15 - 10));
    output[22 + outpos] = (input[10 + inpos] >> 10) & 32767;
    output[23 + outpos] = (input[10 + inpos] >> 25) | ((input[11 + inpos] & 255) << (15 - 8));
    output[24 + outpos] = (input[11 + inpos] >> 8) & 32767;
    output[25 + outpos] = (input[11 + inpos] >> 23) | ((input[12 + inpos] & 63) << (15 - 6));
    output[26 + outpos] = (input[12 + inpos] >> 6) & 32767;
    output[27 + outpos] = (input[12 + inpos] >> 21) | ((input[13 + inpos] & 15) << (15 - 4));
    output[28 + outpos] = (input[13 + inpos] >> 4) & 32767;
    output[29 + outpos] = (input[13 + inpos] >> 19) | ((input[14 + inpos] & 3) << (15 - 2));
    output[30 + outpos] = (input[14 + inpos] >> 2) & 32767;
    output[31 + outpos] = input[14 + inpos] >> 17;
}

fn fast_unpack16(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 65535;
    output[1 + outpos] = input[0 + inpos] >> 16;
    output[2 + outpos] = (input[1 + inpos] >> 0) & 65535;
    output[3 + outpos] = input[1 + inpos] >> 16;
    output[4 + outpos] = (input[2 + inpos] >> 0) & 65535;
    output[5 + outpos] = input[2 + inpos] >> 16;
    output[6 + outpos] = (input[3 + inpos] >> 0) & 65535;
    output[7 + outpos] = input[3 + inpos] >> 16;
    output[8 + outpos] = (input[4 + inpos] >> 0) & 65535;
    output[9 + outpos] = input[4 + inpos] >> 16;
    output[10 + outpos] = (input[5 + inpos] >> 0) & 65535;
    output[11 + outpos] = input[5 + inpos] >> 16;
    output[12 + outpos] = (input[6 + inpos] >> 0) & 65535;
    output[13 + outpos] = input[6 + inpos] >> 16;
    output[14 + outpos] = (input[7 + inpos] >> 0) & 65535;
    output[15 + outpos] = input[7 + inpos] >> 16;
    output[16 + outpos] = (input[8 + inpos] >> 0) & 65535;
    output[17 + outpos] = input[8 + inpos] >> 16;
    output[18 + outpos] = (input[9 + inpos] >> 0) & 65535;
    output[19 + outpos] = input[9 + inpos] >> 16;
    output[20 + outpos] = (input[10 + inpos] >> 0) & 65535;
    output[21 + outpos] = input[10 + inpos] >> 16;
    output[22 + outpos] = (input[11 + inpos] >> 0) & 65535;
    output[23 + outpos] = input[11 + inpos] >> 16;
    output[24 + outpos] = (input[12 + inpos] >> 0) & 65535;
    output[25 + outpos] = input[12 + inpos] >> 16;
    output[26 + outpos] = (input[13 + inpos] >> 0) & 65535;
    output[27 + outpos] = input[13 + inpos] >> 16;
    output[28 + outpos] = (input[14 + inpos] >> 0) & 65535;
    output[29 + outpos] = input[14 + inpos] >> 16;
    output[30 + outpos] = (input[15 + inpos] >> 0) & 65535;
    output[31 + outpos] = input[15 + inpos] >> 16;
}

fn fast_unpack17(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 131071;
    output[1 + outpos] = (input[0 + inpos] >> 17) | ((input[1 + inpos] & 3) << (17 - 2));
    output[2 + outpos] = (input[1 + inpos] >> 2) & 131071;
    output[3 + outpos] = (input[1 + inpos] >> 19) | ((input[2 + inpos] & 15) << (17 - 4));
    output[4 + outpos] = (input[2 + inpos] >> 4) & 131071;
    output[5 + outpos] = (input[2 + inpos] >> 21) | ((input[3 + inpos] & 63) << (17 - 6));
    output[6 + outpos] = (input[3 + inpos] >> 6) & 131071;
    output[7 + outpos] = (input[3 + inpos] >> 23) | ((input[4 + inpos] & 255) << (17 - 8));
    output[8 + outpos] = (input[4 + inpos] >> 8) & 131071;
    output[9 + outpos] = (input[4 + inpos] >> 25) | ((input[5 + inpos] & 1023) << (17 - 10));
    output[10 + outpos] = (input[5 + inpos] >> 10) & 131071;
    output[11 + outpos] = (input[5 + inpos] >> 27) | ((input[6 + inpos] & 4095) << (17 - 12));
    output[12 + outpos] = (input[6 + inpos] >> 12) & 131071;
    output[13 + outpos] = (input[6 + inpos] >> 29) | ((input[7 + inpos] & 16383) << (17 - 14));
    output[14 + outpos] = (input[7 + inpos] >> 14) & 131071;
    output[15 + outpos] = (input[7 + inpos] >> 31) | ((input[8 + inpos] & 65535) << (17 - 16));
    output[16 + outpos] = (input[8 + inpos] >> 16) | ((input[9 + inpos] & 1) << (17 - 1));
    output[17 + outpos] = (input[9 + inpos] >> 1) & 131071;
    output[18 + outpos] = (input[9 + inpos] >> 18) | ((input[10 + inpos] & 7) << (17 - 3));
    output[19 + outpos] = (input[10 + inpos] >> 3) & 131071;
    output[20 + outpos] = (input[10 + inpos] >> 20) | ((input[11 + inpos] & 31) << (17 - 5));
    output[21 + outpos] = (input[11 + inpos] >> 5) & 131071;
    output[22 + outpos] = (input[11 + inpos] >> 22) | ((input[12 + inpos] & 127) << (17 - 7));
    output[23 + outpos] = (input[12 + inpos] >> 7) & 131071;
    output[24 + outpos] = (input[12 + inpos] >> 24) | ((input[13 + inpos] & 511) << (17 - 9));
    output[25 + outpos] = (input[13 + inpos] >> 9) & 131071;
    output[26 + outpos] = (input[13 + inpos] >> 26) | ((input[14 + inpos] & 2047) << (17 - 11));
    output[27 + outpos] = (input[14 + inpos] >> 11) & 131071;
    output[28 + outpos] = (input[14 + inpos] >> 28) | ((input[15 + inpos] & 8191) << (17 - 13));
    output[29 + outpos] = (input[15 + inpos] >> 13) & 131071;
    output[30 + outpos] = (input[15 + inpos] >> 30) | ((input[16 + inpos] & 32767) << (17 - 15));
    output[31 + outpos] = input[16 + inpos] >> 15;
}

fn fast_unpack18(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 262143;
    output[1 + outpos] = (input[0 + inpos] >> 18) | ((input[1 + inpos] & 15) << (18 - 4));
    output[2 + outpos] = (input[1 + inpos] >> 4) & 262143;
    output[3 + outpos] = (input[1 + inpos] >> 22) | ((input[2 + inpos] & 255) << (18 - 8));
    output[4 + outpos] = (input[2 + inpos] >> 8) & 262143;
    output[5 + outpos] = (input[2 + inpos] >> 26) | ((input[3 + inpos] & 4095) << (18 - 12));
    output[6 + outpos] = (input[3 + inpos] >> 12) & 262143;
    output[7 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 65535) << (18 - 16));
    output[8 + outpos] = (input[4 + inpos] >> 16) | ((input[5 + inpos] & 3) << (18 - 2));
    output[9 + outpos] = (input[5 + inpos] >> 2) & 262143;
    output[10 + outpos] = (input[5 + inpos] >> 20) | ((input[6 + inpos] & 63) << (18 - 6));
    output[11 + outpos] = (input[6 + inpos] >> 6) & 262143;
    output[12 + outpos] = (input[6 + inpos] >> 24) | ((input[7 + inpos] & 1023) << (18 - 10));
    output[13 + outpos] = (input[7 + inpos] >> 10) & 262143;
    output[14 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 16383) << (18 - 14));
    output[15 + outpos] = input[8 + inpos] >> 14;
    output[16 + outpos] = (input[9 + inpos] >> 0) & 262143;
    output[17 + outpos] = (input[9 + inpos] >> 18) | ((input[10 + inpos] & 15) << (18 - 4));
    output[18 + outpos] = (input[10 + inpos] >> 4) & 262143;
    output[19 + outpos] = (input[10 + inpos] >> 22) | ((input[11 + inpos] & 255) << (18 - 8));
    output[20 + outpos] = (input[11 + inpos] >> 8) & 262143;
    output[21 + outpos] = (input[11 + inpos] >> 26) | ((input[12 + inpos] & 4095) << (18 - 12));
    output[22 + outpos] = (input[12 + inpos] >> 12) & 262143;
    output[23 + outpos] = (input[12 + inpos] >> 30) | ((input[13 + inpos] & 65535) << (18 - 16));
    output[24 + outpos] = (input[13 + inpos] >> 16) | ((input[14 + inpos] & 3) << (18 - 2));
    output[25 + outpos] = (input[14 + inpos] >> 2) & 262143;
    output[26 + outpos] = (input[14 + inpos] >> 20) | ((input[15 + inpos] & 63) << (18 - 6));
    output[27 + outpos] = (input[15 + inpos] >> 6) & 262143;
    output[28 + outpos] = (input[15 + inpos] >> 24) | ((input[16 + inpos] & 1023) << (18 - 10));
    output[29 + outpos] = (input[16 + inpos] >> 10) & 262143;
    output[30 + outpos] = (input[16 + inpos] >> 28) | ((input[17 + inpos] & 16383) << (18 - 14));
    output[31 + outpos] = input[17 + inpos] >> 14;
}

fn fast_unpack19(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 524287;
    output[1 + outpos] = (input[0 + inpos] >> 19) | ((input[1 + inpos] & 63) << (19 - 6));
    output[2 + outpos] = (input[1 + inpos] >> 6) & 524287;
    output[3 + outpos] = (input[1 + inpos] >> 25) | ((input[2 + inpos] & 4095) << (19 - 12));
    output[4 + outpos] = (input[2 + inpos] >> 12) & 524287;
    output[5 + outpos] = (input[2 + inpos] >> 31) | ((input[3 + inpos] & 262143) << (19 - 18));
    output[6 + outpos] = (input[3 + inpos] >> 18) | ((input[4 + inpos] & 31) << (19 - 5));
    output[7 + outpos] = (input[4 + inpos] >> 5) & 524287;
    output[8 + outpos] = (input[4 + inpos] >> 24) | ((input[5 + inpos] & 2047) << (19 - 11));
    output[9 + outpos] = (input[5 + inpos] >> 11) & 524287;
    output[10 + outpos] = (input[5 + inpos] >> 30) | ((input[6 + inpos] & 131071) << (19 - 17));
    output[11 + outpos] = (input[6 + inpos] >> 17) | ((input[7 + inpos] & 15) << (19 - 4));
    output[12 + outpos] = (input[7 + inpos] >> 4) & 524287;
    output[13 + outpos] = (input[7 + inpos] >> 23) | ((input[8 + inpos] & 1023) << (19 - 10));
    output[14 + outpos] = (input[8 + inpos] >> 10) & 524287;
    output[15 + outpos] = (input[8 + inpos] >> 29) | ((input[9 + inpos] & 65535) << (19 - 16));
    output[16 + outpos] = (input[9 + inpos] >> 16) | ((input[10 + inpos] & 7) << (19 - 3));
    output[17 + outpos] = (input[10 + inpos] >> 3) & 524287;
    output[18 + outpos] = (input[10 + inpos] >> 22) | ((input[11 + inpos] & 511) << (19 - 9));
    output[19 + outpos] = (input[11 + inpos] >> 9) & 524287;
    output[20 + outpos] = (input[11 + inpos] >> 28) | ((input[12 + inpos] & 32767) << (19 - 15));
    output[21 + outpos] = (input[12 + inpos] >> 15) | ((input[13 + inpos] & 3) << (19 - 2));
    output[22 + outpos] = (input[13 + inpos] >> 2) & 524287;
    output[23 + outpos] = (input[13 + inpos] >> 21) | ((input[14 + inpos] & 255) << (19 - 8));
    output[24 + outpos] = (input[14 + inpos] >> 8) & 524287;
    output[25 + outpos] = (input[14 + inpos] >> 27) | ((input[15 + inpos] & 16383) << (19 - 14));
    output[26 + outpos] = (input[15 + inpos] >> 14) | ((input[16 + inpos] & 1) << (19 - 1));
    output[27 + outpos] = (input[16 + inpos] >> 1) & 524287;
    output[28 + outpos] = (input[16 + inpos] >> 20) | ((input[17 + inpos] & 127) << (19 - 7));
    output[29 + outpos] = (input[17 + inpos] >> 7) & 524287;
    output[30 + outpos] = (input[17 + inpos] >> 26) | ((input[18 + inpos] & 8191) << (19 - 13));
    output[31 + outpos] = input[18 + inpos] >> 13;
}

fn fast_unpack20(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 1048575;
    output[1 + outpos] = (input[0 + inpos] >> 20) | ((input[1 + inpos] & 255) << (20 - 8));
    output[2 + outpos] = (input[1 + inpos] >> 8) & 1048575;
    output[3 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 65535) << (20 - 16));
    output[4 + outpos] = (input[2 + inpos] >> 16) | ((input[3 + inpos] & 15) << (20 - 4));
    output[5 + outpos] = (input[3 + inpos] >> 4) & 1048575;
    output[6 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 4095) << (20 - 12));
    output[7 + outpos] = input[4 + inpos] >> 12;
    output[8 + outpos] = (input[5 + inpos] >> 0) & 1048575;
    output[9 + outpos] = (input[5 + inpos] >> 20) | ((input[6 + inpos] & 255) << (20 - 8));
    output[10 + outpos] = (input[6 + inpos] >> 8) & 1048575;
    output[11 + outpos] = (input[6 + inpos] >> 28) | ((input[7 + inpos] & 65535) << (20 - 16));
    output[12 + outpos] = (input[7 + inpos] >> 16) | ((input[8 + inpos] & 15) << (20 - 4));
    output[13 + outpos] = (input[8 + inpos] >> 4) & 1048575;
    output[14 + outpos] = (input[8 + inpos] >> 24) | ((input[9 + inpos] & 4095) << (20 - 12));
    output[15 + outpos] = input[9 + inpos] >> 12;
    output[16 + outpos] = (input[10 + inpos] >> 0) & 1048575;
    output[17 + outpos] = (input[10 + inpos] >> 20) | ((input[11 + inpos] & 255) << (20 - 8));
    output[18 + outpos] = (input[11 + inpos] >> 8) & 1048575;
    output[19 + outpos] = (input[11 + inpos] >> 28) | ((input[12 + inpos] & 65535) << (20 - 16));
    output[20 + outpos] = (input[12 + inpos] >> 16) | ((input[13 + inpos] & 15) << (20 - 4));
    output[21 + outpos] = (input[13 + inpos] >> 4) & 1048575;
    output[22 + outpos] = (input[13 + inpos] >> 24) | ((input[14 + inpos] & 4095) << (20 - 12));
    output[23 + outpos] = input[14 + inpos] >> 12;
    output[24 + outpos] = (input[15 + inpos] >> 0) & 1048575;
    output[25 + outpos] = (input[15 + inpos] >> 20) | ((input[16 + inpos] & 255) << (20 - 8));
    output[26 + outpos] = (input[16 + inpos] >> 8) & 1048575;
    output[27 + outpos] = (input[16 + inpos] >> 28) | ((input[17 + inpos] & 65535) << (20 - 16));
    output[28 + outpos] = (input[17 + inpos] >> 16) | ((input[18 + inpos] & 15) << (20 - 4));
    output[29 + outpos] = (input[18 + inpos] >> 4) & 1048575;
    output[30 + outpos] = (input[18 + inpos] >> 24) | ((input[19 + inpos] & 4095) << (20 - 12));
    output[31 + outpos] = input[19 + inpos] >> 12;
}

fn fast_unpack21(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 2097151;
    output[1 + outpos] = (input[0 + inpos] >> 21) | ((input[1 + inpos] & 1023) << (21 - 10));
    output[2 + outpos] = (input[1 + inpos] >> 10) & 2097151;
    output[3 + outpos] = (input[1 + inpos] >> 31) | ((input[2 + inpos] & 1048575) << (21 - 20));
    output[4 + outpos] = (input[2 + inpos] >> 20) | ((input[3 + inpos] & 511) << (21 - 9));
    output[5 + outpos] = (input[3 + inpos] >> 9) & 2097151;
    output[6 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 524287) << (21 - 19));
    output[7 + outpos] = (input[4 + inpos] >> 19) | ((input[5 + inpos] & 255) << (21 - 8));
    output[8 + outpos] = (input[5 + inpos] >> 8) & 2097151;
    output[9 + outpos] = (input[5 + inpos] >> 29) | ((input[6 + inpos] & 262143) << (21 - 18));
    output[10 + outpos] = (input[6 + inpos] >> 18) | ((input[7 + inpos] & 127) << (21 - 7));
    output[11 + outpos] = (input[7 + inpos] >> 7) & 2097151;
    output[12 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 131071) << (21 - 17));
    output[13 + outpos] = (input[8 + inpos] >> 17) | ((input[9 + inpos] & 63) << (21 - 6));
    output[14 + outpos] = (input[9 + inpos] >> 6) & 2097151;
    output[15 + outpos] = (input[9 + inpos] >> 27) | ((input[10 + inpos] & 65535) << (21 - 16));
    output[16 + outpos] = (input[10 + inpos] >> 16) | ((input[11 + inpos] & 31) << (21 - 5));
    output[17 + outpos] = (input[11 + inpos] >> 5) & 2097151;
    output[18 + outpos] = (input[11 + inpos] >> 26) | ((input[12 + inpos] & 32767) << (21 - 15));
    output[19 + outpos] = (input[12 + inpos] >> 15) | ((input[13 + inpos] & 15) << (21 - 4));
    output[20 + outpos] = (input[13 + inpos] >> 4) & 2097151;
    output[21 + outpos] = (input[13 + inpos] >> 25) | ((input[14 + inpos] & 16383) << (21 - 14));
    output[22 + outpos] = (input[14 + inpos] >> 14) | ((input[15 + inpos] & 7) << (21 - 3));
    output[23 + outpos] = (input[15 + inpos] >> 3) & 2097151;
    output[24 + outpos] = (input[15 + inpos] >> 24) | ((input[16 + inpos] & 8191) << (21 - 13));
    output[25 + outpos] = (input[16 + inpos] >> 13) | ((input[17 + inpos] & 3) << (21 - 2));
    output[26 + outpos] = (input[17 + inpos] >> 2) & 2097151;
    output[27 + outpos] = (input[17 + inpos] >> 23) | ((input[18 + inpos] & 4095) << (21 - 12));
    output[28 + outpos] = (input[18 + inpos] >> 12) | ((input[19 + inpos] & 1) << (21 - 1));
    output[29 + outpos] = (input[19 + inpos] >> 1) & 2097151;
    output[30 + outpos] = (input[19 + inpos] >> 22) | ((input[20 + inpos] & 2047) << (21 - 11));
    output[31 + outpos] = input[20 + inpos] >> 11;
}

fn fast_unpack22(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 4194303;
    output[1 + outpos] = (input[0 + inpos] >> 22) | ((input[1 + inpos] & 4095) << (22 - 12));
    output[2 + outpos] = (input[1 + inpos] >> 12) | ((input[2 + inpos] & 3) << (22 - 2));
    output[3 + outpos] = (input[2 + inpos] >> 2) & 4194303;
    output[4 + outpos] = (input[2 + inpos] >> 24) | ((input[3 + inpos] & 16383) << (22 - 14));
    output[5 + outpos] = (input[3 + inpos] >> 14) | ((input[4 + inpos] & 15) << (22 - 4));
    output[6 + outpos] = (input[4 + inpos] >> 4) & 4194303;
    output[7 + outpos] = (input[4 + inpos] >> 26) | ((input[5 + inpos] & 65535) << (22 - 16));
    output[8 + outpos] = (input[5 + inpos] >> 16) | ((input[6 + inpos] & 63) << (22 - 6));
    output[9 + outpos] = (input[6 + inpos] >> 6) & 4194303;
    output[10 + outpos] = (input[6 + inpos] >> 28) | ((input[7 + inpos] & 262143) << (22 - 18));
    output[11 + outpos] = (input[7 + inpos] >> 18) | ((input[8 + inpos] & 255) << (22 - 8));
    output[12 + outpos] = (input[8 + inpos] >> 8) & 4194303;
    output[13 + outpos] = (input[8 + inpos] >> 30) | ((input[9 + inpos] & 1048575) << (22 - 20));
    output[14 + outpos] = (input[9 + inpos] >> 20) | ((input[10 + inpos] & 1023) << (22 - 10));
    output[15 + outpos] = input[10 + inpos] >> 10;
    output[16 + outpos] = (input[11 + inpos] >> 0) & 4194303;
    output[17 + outpos] = (input[11 + inpos] >> 22) | ((input[12 + inpos] & 4095) << (22 - 12));
    output[18 + outpos] = (input[12 + inpos] >> 12) | ((input[13 + inpos] & 3) << (22 - 2));
    output[19 + outpos] = (input[13 + inpos] >> 2) & 4194303;
    output[20 + outpos] = (input[13 + inpos] >> 24) | ((input[14 + inpos] & 16383) << (22 - 14));
    output[21 + outpos] = (input[14 + inpos] >> 14) | ((input[15 + inpos] & 15) << (22 - 4));
    output[22 + outpos] = (input[15 + inpos] >> 4) & 4194303;
    output[23 + outpos] = (input[15 + inpos] >> 26) | ((input[16 + inpos] & 65535) << (22 - 16));
    output[24 + outpos] = (input[16 + inpos] >> 16) | ((input[17 + inpos] & 63) << (22 - 6));
    output[25 + outpos] = (input[17 + inpos] >> 6) & 4194303;
    output[26 + outpos] = (input[17 + inpos] >> 28) | ((input[18 + inpos] & 262143) << (22 - 18));
    output[27 + outpos] = (input[18 + inpos] >> 18) | ((input[19 + inpos] & 255) << (22 - 8));
    output[28 + outpos] = (input[19 + inpos] >> 8) & 4194303;
    output[29 + outpos] = (input[19 + inpos] >> 30) | ((input[20 + inpos] & 1048575) << (22 - 20));
    output[30 + outpos] = (input[20 + inpos] >> 20) | ((input[21 + inpos] & 1023) << (22 - 10));
    output[31 + outpos] = input[21 + inpos] >> 10;
}

fn fast_unpack23(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 8388607;
    output[1 + outpos] = (input[0 + inpos] >> 23) | ((input[1 + inpos] & 16383) << (23 - 14));
    output[2 + outpos] = (input[1 + inpos] >> 14) | ((input[2 + inpos] & 31) << (23 - 5));
    output[3 + outpos] = (input[2 + inpos] >> 5) & 8388607;
    output[4 + outpos] = (input[2 + inpos] >> 28) | ((input[3 + inpos] & 524287) << (23 - 19));
    output[5 + outpos] = (input[3 + inpos] >> 19) | ((input[4 + inpos] & 1023) << (23 - 10));
    output[6 + outpos] = (input[4 + inpos] >> 10) | ((input[5 + inpos] & 1) << (23 - 1));
    output[7 + outpos] = (input[5 + inpos] >> 1) & 8388607;
    output[8 + outpos] = (input[5 + inpos] >> 24) | ((input[6 + inpos] & 32767) << (23 - 15));
    output[9 + outpos] = (input[6 + inpos] >> 15) | ((input[7 + inpos] & 63) << (23 - 6));
    output[10 + outpos] = (input[7 + inpos] >> 6) & 8388607;
    output[11 + outpos] = (input[7 + inpos] >> 29) | ((input[8 + inpos] & 1048575) << (23 - 20));
    output[12 + outpos] = (input[8 + inpos] >> 20) | ((input[9 + inpos] & 2047) << (23 - 11));
    output[13 + outpos] = (input[9 + inpos] >> 11) | ((input[10 + inpos] & 3) << (23 - 2));
    output[14 + outpos] = (input[10 + inpos] >> 2) & 8388607;
    output[15 + outpos] = (input[10 + inpos] >> 25) | ((input[11 + inpos] & 65535) << (23 - 16));
    output[16 + outpos] = (input[11 + inpos] >> 16) | ((input[12 + inpos] & 127) << (23 - 7));
    output[17 + outpos] = (input[12 + inpos] >> 7) & 8388607;
    output[18 + outpos] = (input[12 + inpos] >> 30) | ((input[13 + inpos] & 2097151) << (23 - 21));
    output[19 + outpos] = (input[13 + inpos] >> 21) | ((input[14 + inpos] & 4095) << (23 - 12));
    output[20 + outpos] = (input[14 + inpos] >> 12) | ((input[15 + inpos] & 7) << (23 - 3));
    output[21 + outpos] = (input[15 + inpos] >> 3) & 8388607;
    output[22 + outpos] = (input[15 + inpos] >> 26) | ((input[16 + inpos] & 131071) << (23 - 17));
    output[23 + outpos] = (input[16 + inpos] >> 17) | ((input[17 + inpos] & 255) << (23 - 8));
    output[24 + outpos] = (input[17 + inpos] >> 8) & 8388607;
    output[25 + outpos] = (input[17 + inpos] >> 31) | ((input[18 + inpos] & 4194303) << (23 - 22));
    output[26 + outpos] = (input[18 + inpos] >> 22) | ((input[19 + inpos] & 8191) << (23 - 13));
    output[27 + outpos] = (input[19 + inpos] >> 13) | ((input[20 + inpos] & 15) << (23 - 4));
    output[28 + outpos] = (input[20 + inpos] >> 4) & 8388607;
    output[29 + outpos] = (input[20 + inpos] >> 27) | ((input[21 + inpos] & 262143) << (23 - 18));
    output[30 + outpos] = (input[21 + inpos] >> 18) | ((input[22 + inpos] & 511) << (23 - 9));
    output[31 + outpos] = input[22 + inpos] >> 9;
}

fn fast_unpack24(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 16777215;
    output[1 + outpos] = (input[0 + inpos] >> 24) | ((input[1 + inpos] & 65535) << (24 - 16));
    output[2 + outpos] = (input[1 + inpos] >> 16) | ((input[2 + inpos] & 255) << (24 - 8));
    output[3 + outpos] = input[2 + inpos] >> 8;
    output[4 + outpos] = (input[3 + inpos] >> 0) & 16777215;
    output[5 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 65535) << (24 - 16));
    output[6 + outpos] = (input[4 + inpos] >> 16) | ((input[5 + inpos] & 255) << (24 - 8));
    output[7 + outpos] = input[5 + inpos] >> 8;
    output[8 + outpos] = (input[6 + inpos] >> 0) & 16777215;
    output[9 + outpos] = (input[6 + inpos] >> 24) | ((input[7 + inpos] & 65535) << (24 - 16));
    output[10 + outpos] = (input[7 + inpos] >> 16) | ((input[8 + inpos] & 255) << (24 - 8));
    output[11 + outpos] = input[8 + inpos] >> 8;
    output[12 + outpos] = (input[9 + inpos] >> 0) & 16777215;
    output[13 + outpos] = (input[9 + inpos] >> 24) | ((input[10 + inpos] & 65535) << (24 - 16));
    output[14 + outpos] = (input[10 + inpos] >> 16) | ((input[11 + inpos] & 255) << (24 - 8));
    output[15 + outpos] = input[11 + inpos] >> 8;
    output[16 + outpos] = (input[12 + inpos] >> 0) & 16777215;
    output[17 + outpos] = (input[12 + inpos] >> 24) | ((input[13 + inpos] & 65535) << (24 - 16));
    output[18 + outpos] = (input[13 + inpos] >> 16) | ((input[14 + inpos] & 255) << (24 - 8));
    output[19 + outpos] = input[14 + inpos] >> 8;
    output[20 + outpos] = (input[15 + inpos] >> 0) & 16777215;
    output[21 + outpos] = (input[15 + inpos] >> 24) | ((input[16 + inpos] & 65535) << (24 - 16));
    output[22 + outpos] = (input[16 + inpos] >> 16) | ((input[17 + inpos] & 255) << (24 - 8));
    output[23 + outpos] = input[17 + inpos] >> 8;
    output[24 + outpos] = (input[18 + inpos] >> 0) & 16777215;
    output[25 + outpos] = (input[18 + inpos] >> 24) | ((input[19 + inpos] & 65535) << (24 - 16));
    output[26 + outpos] = (input[19 + inpos] >> 16) | ((input[20 + inpos] & 255) << (24 - 8));
    output[27 + outpos] = input[20 + inpos] >> 8;
    output[28 + outpos] = (input[21 + inpos] >> 0) & 16777215;
    output[29 + outpos] = (input[21 + inpos] >> 24) | ((input[22 + inpos] & 65535) << (24 - 16));
    output[30 + outpos] = (input[22 + inpos] >> 16) | ((input[23 + inpos] & 255) << (24 - 8));
    output[31 + outpos] = input[23 + inpos] >> 8;
}

fn fast_unpack25(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 33554431;
    output[1 + outpos] = (input[0 + inpos] >> 25) | ((input[1 + inpos] & 262143) << (25 - 18));
    output[2 + outpos] = (input[1 + inpos] >> 18) | ((input[2 + inpos] & 2047) << (25 - 11));
    output[3 + outpos] = (input[2 + inpos] >> 11) | ((input[3 + inpos] & 15) << (25 - 4));
    output[4 + outpos] = (input[3 + inpos] >> 4) & 33554431;
    output[5 + outpos] = (input[3 + inpos] >> 29) | ((input[4 + inpos] & 4194303) << (25 - 22));
    output[6 + outpos] = (input[4 + inpos] >> 22) | ((input[5 + inpos] & 32767) << (25 - 15));
    output[7 + outpos] = (input[5 + inpos] >> 15) | ((input[6 + inpos] & 255) << (25 - 8));
    output[8 + outpos] = (input[6 + inpos] >> 8) | ((input[7 + inpos] & 1) << (25 - 1));
    output[9 + outpos] = (input[7 + inpos] >> 1) & 33554431;
    output[10 + outpos] = (input[7 + inpos] >> 26) | ((input[8 + inpos] & 524287) << (25 - 19));
    output[11 + outpos] = (input[8 + inpos] >> 19) | ((input[9 + inpos] & 4095) << (25 - 12));
    output[12 + outpos] = (input[9 + inpos] >> 12) | ((input[10 + inpos] & 31) << (25 - 5));
    output[13 + outpos] = (input[10 + inpos] >> 5) & 33554431;
    output[14 + outpos] = (input[10 + inpos] >> 30) | ((input[11 + inpos] & 8388607) << (25 - 23));
    output[15 + outpos] = (input[11 + inpos] >> 23) | ((input[12 + inpos] & 65535) << (25 - 16));
    output[16 + outpos] = (input[12 + inpos] >> 16) | ((input[13 + inpos] & 511) << (25 - 9));
    output[17 + outpos] = (input[13 + inpos] >> 9) | ((input[14 + inpos] & 3) << (25 - 2));
    output[18 + outpos] = (input[14 + inpos] >> 2) & 33554431;
    output[19 + outpos] = (input[14 + inpos] >> 27) | ((input[15 + inpos] & 1048575) << (25 - 20));
    output[20 + outpos] = (input[15 + inpos] >> 20) | ((input[16 + inpos] & 8191) << (25 - 13));
    output[21 + outpos] = (input[16 + inpos] >> 13) | ((input[17 + inpos] & 63) << (25 - 6));
    output[22 + outpos] = (input[17 + inpos] >> 6) & 33554431;
    output[23 + outpos] = (input[17 + inpos] >> 31) | ((input[18 + inpos] & 16777215) << (25 - 24));
    output[24 + outpos] = (input[18 + inpos] >> 24) | ((input[19 + inpos] & 131071) << (25 - 17));
    output[25 + outpos] = (input[19 + inpos] >> 17) | ((input[20 + inpos] & 1023) << (25 - 10));
    output[26 + outpos] = (input[20 + inpos] >> 10) | ((input[21 + inpos] & 7) << (25 - 3));
    output[27 + outpos] = (input[21 + inpos] >> 3) & 33554431;
    output[28 + outpos] = (input[21 + inpos] >> 28) | ((input[22 + inpos] & 2097151) << (25 - 21));
    output[29 + outpos] = (input[22 + inpos] >> 21) | ((input[23 + inpos] & 16383) << (25 - 14));
    output[30 + outpos] = (input[23 + inpos] >> 14) | ((input[24 + inpos] & 127) << (25 - 7));
    output[31 + outpos] = input[24 + inpos] >> 7;
}

fn fast_unpack26(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 67108863;
    output[1 + outpos] = (input[0 + inpos] >> 26) | ((input[1 + inpos] & 1048575) << (26 - 20));
    output[2 + outpos] = (input[1 + inpos] >> 20) | ((input[2 + inpos] & 16383) << (26 - 14));
    output[3 + outpos] = (input[2 + inpos] >> 14) | ((input[3 + inpos] & 255) << (26 - 8));
    output[4 + outpos] = (input[3 + inpos] >> 8) | ((input[4 + inpos] & 3) << (26 - 2));
    output[5 + outpos] = (input[4 + inpos] >> 2) & 67108863;
    output[6 + outpos] = (input[4 + inpos] >> 28) | ((input[5 + inpos] & 4194303) << (26 - 22));
    output[7 + outpos] = (input[5 + inpos] >> 22) | ((input[6 + inpos] & 65535) << (26 - 16));
    output[8 + outpos] = (input[6 + inpos] >> 16) | ((input[7 + inpos] & 1023) << (26 - 10));
    output[9 + outpos] = (input[7 + inpos] >> 10) | ((input[8 + inpos] & 15) << (26 - 4));
    output[10 + outpos] = (input[8 + inpos] >> 4) & 67108863;
    output[11 + outpos] = (input[8 + inpos] >> 30) | ((input[9 + inpos] & 16777215) << (26 - 24));
    output[12 + outpos] = (input[9 + inpos] >> 24) | ((input[10 + inpos] & 262143) << (26 - 18));
    output[13 + outpos] = (input[10 + inpos] >> 18) | ((input[11 + inpos] & 4095) << (26 - 12));
    output[14 + outpos] = (input[11 + inpos] >> 12) | ((input[12 + inpos] & 63) << (26 - 6));
    output[15 + outpos] = input[12 + inpos] >> 6;
    output[16 + outpos] = (input[13 + inpos] >> 0) & 67108863;
    output[17 + outpos] = (input[13 + inpos] >> 26) | ((input[14 + inpos] & 1048575) << (26 - 20));
    output[18 + outpos] = (input[14 + inpos] >> 20) | ((input[15 + inpos] & 16383) << (26 - 14));
    output[19 + outpos] = (input[15 + inpos] >> 14) | ((input[16 + inpos] & 255) << (26 - 8));
    output[20 + outpos] = (input[16 + inpos] >> 8) | ((input[17 + inpos] & 3) << (26 - 2));
    output[21 + outpos] = (input[17 + inpos] >> 2) & 67108863;
    output[22 + outpos] = (input[17 + inpos] >> 28) | ((input[18 + inpos] & 4194303) << (26 - 22));
    output[23 + outpos] = (input[18 + inpos] >> 22) | ((input[19 + inpos] & 65535) << (26 - 16));
    output[24 + outpos] = (input[19 + inpos] >> 16) | ((input[20 + inpos] & 1023) << (26 - 10));
    output[25 + outpos] = (input[20 + inpos] >> 10) | ((input[21 + inpos] & 15) << (26 - 4));
    output[26 + outpos] = (input[21 + inpos] >> 4) & 67108863;
    output[27 + outpos] = (input[21 + inpos] >> 30) | ((input[22 + inpos] & 16777215) << (26 - 24));
    output[28 + outpos] = (input[22 + inpos] >> 24) | ((input[23 + inpos] & 262143) << (26 - 18));
    output[29 + outpos] = (input[23 + inpos] >> 18) | ((input[24 + inpos] & 4095) << (26 - 12));
    output[30 + outpos] = (input[24 + inpos] >> 12) | ((input[25 + inpos] & 63) << (26 - 6));
    output[31 + outpos] = input[25 + inpos] >> 6;
}

fn fast_unpack27(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 134217727;
    output[1 + outpos] = (input[0 + inpos] >> 27) | ((input[1 + inpos] & 4194303) << (27 - 22));
    output[2 + outpos] = (input[1 + inpos] >> 22) | ((input[2 + inpos] & 131071) << (27 - 17));
    output[3 + outpos] = (input[2 + inpos] >> 17) | ((input[3 + inpos] & 4095) << (27 - 12));
    output[4 + outpos] = (input[3 + inpos] >> 12) | ((input[4 + inpos] & 127) << (27 - 7));
    output[5 + outpos] = (input[4 + inpos] >> 7) | ((input[5 + inpos] & 3) << (27 - 2));
    output[6 + outpos] = (input[5 + inpos] >> 2) & 134217727;
    output[7 + outpos] = (input[5 + inpos] >> 29) | ((input[6 + inpos] & 16777215) << (27 - 24));
    output[8 + outpos] = (input[6 + inpos] >> 24) | ((input[7 + inpos] & 524287) << (27 - 19));
    output[9 + outpos] = (input[7 + inpos] >> 19) | ((input[8 + inpos] & 16383) << (27 - 14));
    output[10 + outpos] = (input[8 + inpos] >> 14) | ((input[9 + inpos] & 511) << (27 - 9));
    output[11 + outpos] = (input[9 + inpos] >> 9) | ((input[10 + inpos] & 15) << (27 - 4));
    output[12 + outpos] = (input[10 + inpos] >> 4) & 134217727;
    output[13 + outpos] = (input[10 + inpos] >> 31) | ((input[11 + inpos] & 67108863) << (27 - 26));
    output[14 + outpos] = (input[11 + inpos] >> 26) | ((input[12 + inpos] & 2097151) << (27 - 21));
    output[15 + outpos] = (input[12 + inpos] >> 21) | ((input[13 + inpos] & 65535) << (27 - 16));
    output[16 + outpos] = (input[13 + inpos] >> 16) | ((input[14 + inpos] & 2047) << (27 - 11));
    output[17 + outpos] = (input[14 + inpos] >> 11) | ((input[15 + inpos] & 63) << (27 - 6));
    output[18 + outpos] = (input[15 + inpos] >> 6) | ((input[16 + inpos] & 1) << (27 - 1));
    output[19 + outpos] = (input[16 + inpos] >> 1) & 134217727;
    output[20 + outpos] = (input[16 + inpos] >> 28) | ((input[17 + inpos] & 8388607) << (27 - 23));
    output[21 + outpos] = (input[17 + inpos] >> 23) | ((input[18 + inpos] & 262143) << (27 - 18));
    output[22 + outpos] = (input[18 + inpos] >> 18) | ((input[19 + inpos] & 8191) << (27 - 13));
    output[23 + outpos] = (input[19 + inpos] >> 13) | ((input[20 + inpos] & 255) << (27 - 8));
    output[24 + outpos] = (input[20 + inpos] >> 8) | ((input[21 + inpos] & 7) << (27 - 3));
    output[25 + outpos] = (input[21 + inpos] >> 3) & 134217727;
    output[26 + outpos] = (input[21 + inpos] >> 30) | ((input[22 + inpos] & 33554431) << (27 - 25));
    output[27 + outpos] = (input[22 + inpos] >> 25) | ((input[23 + inpos] & 1048575) << (27 - 20));
    output[28 + outpos] = (input[23 + inpos] >> 20) | ((input[24 + inpos] & 32767) << (27 - 15));
    output[29 + outpos] = (input[24 + inpos] >> 15) | ((input[25 + inpos] & 1023) << (27 - 10));
    output[30 + outpos] = (input[25 + inpos] >> 10) | ((input[26 + inpos] & 31) << (27 - 5));
    output[31 + outpos] = input[26 + inpos] >> 5;
}

fn fast_unpack28(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 268435455;
    output[1 + outpos] = (input[0 + inpos] >> 28) | ((input[1 + inpos] & 16777215) << (28 - 24));
    output[2 + outpos] = (input[1 + inpos] >> 24) | ((input[2 + inpos] & 1048575) << (28 - 20));
    output[3 + outpos] = (input[2 + inpos] >> 20) | ((input[3 + inpos] & 65535) << (28 - 16));
    output[4 + outpos] = (input[3 + inpos] >> 16) | ((input[4 + inpos] & 4095) << (28 - 12));
    output[5 + outpos] = (input[4 + inpos] >> 12) | ((input[5 + inpos] & 255) << (28 - 8));
    output[6 + outpos] = (input[5 + inpos] >> 8) | ((input[6 + inpos] & 15) << (28 - 4));
    output[7 + outpos] = input[6 + inpos] >> 4;
    output[8 + outpos] = (input[7 + inpos] >> 0) & 268435455;
    output[9 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 16777215) << (28 - 24));
    output[10 + outpos] = (input[8 + inpos] >> 24) | ((input[9 + inpos] & 1048575) << (28 - 20));
    output[11 + outpos] = (input[9 + inpos] >> 20) | ((input[10 + inpos] & 65535) << (28 - 16));
    output[12 + outpos] = (input[10 + inpos] >> 16) | ((input[11 + inpos] & 4095) << (28 - 12));
    output[13 + outpos] = (input[11 + inpos] >> 12) | ((input[12 + inpos] & 255) << (28 - 8));
    output[14 + outpos] = (input[12 + inpos] >> 8) | ((input[13 + inpos] & 15) << (28 - 4));
    output[15 + outpos] = input[13 + inpos] >> 4;
    output[16 + outpos] = (input[14 + inpos] >> 0) & 268435455;
    output[17 + outpos] = (input[14 + inpos] >> 28) | ((input[15 + inpos] & 16777215) << (28 - 24));
    output[18 + outpos] = (input[15 + inpos] >> 24) | ((input[16 + inpos] & 1048575) << (28 - 20));
    output[19 + outpos] = (input[16 + inpos] >> 20) | ((input[17 + inpos] & 65535) << (28 - 16));
    output[20 + outpos] = (input[17 + inpos] >> 16) | ((input[18 + inpos] & 4095) << (28 - 12));
    output[21 + outpos] = (input[18 + inpos] >> 12) | ((input[19 + inpos] & 255) << (28 - 8));
    output[22 + outpos] = (input[19 + inpos] >> 8) | ((input[20 + inpos] & 15) << (28 - 4));
    output[23 + outpos] = input[20 + inpos] >> 4;
    output[24 + outpos] = (input[21 + inpos] >> 0) & 268435455;
    output[25 + outpos] = (input[21 + inpos] >> 28) | ((input[22 + inpos] & 16777215) << (28 - 24));
    output[26 + outpos] = (input[22 + inpos] >> 24) | ((input[23 + inpos] & 1048575) << (28 - 20));
    output[27 + outpos] = (input[23 + inpos] >> 20) | ((input[24 + inpos] & 65535) << (28 - 16));
    output[28 + outpos] = (input[24 + inpos] >> 16) | ((input[25 + inpos] & 4095) << (28 - 12));
    output[29 + outpos] = (input[25 + inpos] >> 12) | ((input[26 + inpos] & 255) << (28 - 8));
    output[30 + outpos] = (input[26 + inpos] >> 8) | ((input[27 + inpos] & 15) << (28 - 4));
    output[31 + outpos] = input[27 + inpos] >> 4;
}

fn fast_unpack29(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 536870911;
    output[1 + outpos] = (input[0 + inpos] >> 29) | ((input[1 + inpos] & 67108863) << (29 - 26));
    output[2 + outpos] = (input[1 + inpos] >> 26) | ((input[2 + inpos] & 8388607) << (29 - 23));
    output[3 + outpos] = (input[2 + inpos] >> 23) | ((input[3 + inpos] & 1048575) << (29 - 20));
    output[4 + outpos] = (input[3 + inpos] >> 20) | ((input[4 + inpos] & 131071) << (29 - 17));
    output[5 + outpos] = (input[4 + inpos] >> 17) | ((input[5 + inpos] & 16383) << (29 - 14));
    output[6 + outpos] = (input[5 + inpos] >> 14) | ((input[6 + inpos] & 2047) << (29 - 11));
    output[7 + outpos] = (input[6 + inpos] >> 11) | ((input[7 + inpos] & 255) << (29 - 8));
    output[8 + outpos] = (input[7 + inpos] >> 8) | ((input[8 + inpos] & 31) << (29 - 5));
    output[9 + outpos] = (input[8 + inpos] >> 5) | ((input[9 + inpos] & 3) << (29 - 2));
    output[10 + outpos] = (input[9 + inpos] >> 2) & 536870911;
    output[11 + outpos] = (input[9 + inpos] >> 31) | ((input[10 + inpos] & 268435455) << (29 - 28));
    output[12 + outpos] = (input[10 + inpos] >> 28) | ((input[11 + inpos] & 33554431) << (29 - 25));
    output[13 + outpos] = (input[11 + inpos] >> 25) | ((input[12 + inpos] & 4194303) << (29 - 22));
    output[14 + outpos] = (input[12 + inpos] >> 22) | ((input[13 + inpos] & 524287) << (29 - 19));
    output[15 + outpos] = (input[13 + inpos] >> 19) | ((input[14 + inpos] & 65535) << (29 - 16));
    output[16 + outpos] = (input[14 + inpos] >> 16) | ((input[15 + inpos] & 8191) << (29 - 13));
    output[17 + outpos] = (input[15 + inpos] >> 13) | ((input[16 + inpos] & 1023) << (29 - 10));
    output[18 + outpos] = (input[16 + inpos] >> 10) | ((input[17 + inpos] & 127) << (29 - 7));
    output[19 + outpos] = (input[17 + inpos] >> 7) | ((input[18 + inpos] & 15) << (29 - 4));
    output[20 + outpos] = (input[18 + inpos] >> 4) | ((input[19 + inpos] & 1) << (29 - 1));
    output[21 + outpos] = (input[19 + inpos] >> 1) & 536870911;
    output[22 + outpos] =
        (input[19 + inpos] >> 30) | ((input[20 + inpos] & 134217727) << (29 - 27));
    output[23 + outpos] = (input[20 + inpos] >> 27) | ((input[21 + inpos] & 16777215) << (29 - 24));
    output[24 + outpos] = (input[21 + inpos] >> 24) | ((input[22 + inpos] & 2097151) << (29 - 21));
    output[25 + outpos] = (input[22 + inpos] >> 21) | ((input[23 + inpos] & 262143) << (29 - 18));
    output[26 + outpos] = (input[23 + inpos] >> 18) | ((input[24 + inpos] & 32767) << (29 - 15));
    output[27 + outpos] = (input[24 + inpos] >> 15) | ((input[25 + inpos] & 4095) << (29 - 12));
    output[28 + outpos] = (input[25 + inpos] >> 12) | ((input[26 + inpos] & 511) << (29 - 9));
    output[29 + outpos] = (input[26 + inpos] >> 9) | ((input[27 + inpos] & 63) << (29 - 6));
    output[30 + outpos] = (input[27 + inpos] >> 6) | ((input[28 + inpos] & 7) << (29 - 3));
    output[31 + outpos] = input[28 + inpos] >> 3;
}

fn fast_unpack30(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 1073741823;
    output[1 + outpos] = (input[0 + inpos] >> 30) | ((input[1 + inpos] & 268435455) << (30 - 28));
    output[2 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 67108863) << (30 - 26));
    output[3 + outpos] = (input[2 + inpos] >> 26) | ((input[3 + inpos] & 16777215) << (30 - 24));
    output[4 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 4194303) << (30 - 22));
    output[5 + outpos] = (input[4 + inpos] >> 22) | ((input[5 + inpos] & 1048575) << (30 - 20));
    output[6 + outpos] = (input[5 + inpos] >> 20) | ((input[6 + inpos] & 262143) << (30 - 18));
    output[7 + outpos] = (input[6 + inpos] >> 18) | ((input[7 + inpos] & 65535) << (30 - 16));
    output[8 + outpos] = (input[7 + inpos] >> 16) | ((input[8 + inpos] & 16383) << (30 - 14));
    output[9 + outpos] = (input[8 + inpos] >> 14) | ((input[9 + inpos] & 4095) << (30 - 12));
    output[10 + outpos] = (input[9 + inpos] >> 12) | ((input[10 + inpos] & 1023) << (30 - 10));
    output[11 + outpos] = (input[10 + inpos] >> 10) | ((input[11 + inpos] & 255) << (30 - 8));
    output[12 + outpos] = (input[11 + inpos] >> 8) | ((input[12 + inpos] & 63) << (30 - 6));
    output[13 + outpos] = (input[12 + inpos] >> 6) | ((input[13 + inpos] & 15) << (30 - 4));
    output[14 + outpos] = (input[13 + inpos] >> 4) | ((input[14 + inpos] & 3) << (30 - 2));
    output[15 + outpos] = input[14 + inpos] >> 2;
    output[16 + outpos] = (input[15 + inpos] >> 0) & 1073741823;
    output[17 + outpos] =
        (input[15 + inpos] >> 30) | ((input[16 + inpos] & 268435455) << (30 - 28));
    output[18 + outpos] = (input[16 + inpos] >> 28) | ((input[17 + inpos] & 67108863) << (30 - 26));
    output[19 + outpos] = (input[17 + inpos] >> 26) | ((input[18 + inpos] & 16777215) << (30 - 24));
    output[20 + outpos] = (input[18 + inpos] >> 24) | ((input[19 + inpos] & 4194303) << (30 - 22));
    output[21 + outpos] = (input[19 + inpos] >> 22) | ((input[20 + inpos] & 1048575) << (30 - 20));
    output[22 + outpos] = (input[20 + inpos] >> 20) | ((input[21 + inpos] & 262143) << (30 - 18));
    output[23 + outpos] = (input[21 + inpos] >> 18) | ((input[22 + inpos] & 65535) << (30 - 16));
    output[24 + outpos] = (input[22 + inpos] >> 16) | ((input[23 + inpos] & 16383) << (30 - 14));
    output[25 + outpos] = (input[23 + inpos] >> 14) | ((input[24 + inpos] & 4095) << (30 - 12));
    output[26 + outpos] = (input[24 + inpos] >> 12) | ((input[25 + inpos] & 1023) << (30 - 10));
    output[27 + outpos] = (input[25 + inpos] >> 10) | ((input[26 + inpos] & 255) << (30 - 8));
    output[28 + outpos] = (input[26 + inpos] >> 8) | ((input[27 + inpos] & 63) << (30 - 6));
    output[29 + outpos] = (input[27 + inpos] >> 6) | ((input[28 + inpos] & 15) << (30 - 4));
    output[30 + outpos] = (input[28 + inpos] >> 4) | ((input[29 + inpos] & 3) << (30 - 2));
    output[31 + outpos] = input[29 + inpos] >> 2;
}

fn fast_unpack31(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 2147483647;
    output[1 + outpos] = (input[0 + inpos] >> 31) | ((input[1 + inpos] & 1073741823) << (31 - 30));
    output[2 + outpos] = (input[1 + inpos] >> 30) | ((input[2 + inpos] & 536870911) << (31 - 29));
    output[3 + outpos] = (input[2 + inpos] >> 29) | ((input[3 + inpos] & 268435455) << (31 - 28));
    output[4 + outpos] = (input[3 + inpos] >> 28) | ((input[4 + inpos] & 134217727) << (31 - 27));
    output[5 + outpos] = (input[4 + inpos] >> 27) | ((input[5 + inpos] & 67108863) << (31 - 26));
    output[6 + outpos] = (input[5 + inpos] >> 26) | ((input[6 + inpos] & 33554431) << (31 - 25));
    output[7 + outpos] = (input[6 + inpos] >> 25) | ((input[7 + inpos] & 16777215) << (31 - 24));
    output[8 + outpos] = (input[7 + inpos] >> 24) | ((input[8 + inpos] & 8388607) << (31 - 23));
    output[9 + outpos] = (input[8 + inpos] >> 23) | ((input[9 + inpos] & 4194303) << (31 - 22));
    output[10 + outpos] = (input[9 + inpos] >> 22) | ((input[10 + inpos] & 2097151) << (31 - 21));
    output[11 + outpos] = (input[10 + inpos] >> 21) | ((input[11 + inpos] & 1048575) << (31 - 20));
    output[12 + outpos] = (input[11 + inpos] >> 20) | ((input[12 + inpos] & 524287) << (31 - 19));
    output[13 + outpos] = (input[12 + inpos] >> 19) | ((input[13 + inpos] & 262143) << (31 - 18));
    output[14 + outpos] = (input[13 + inpos] >> 18) | ((input[14 + inpos] & 131071) << (31 - 17));
    output[15 + outpos] = (input[14 + inpos] >> 17) | ((input[15 + inpos] & 65535) << (31 - 16));
    output[16 + outpos] = (input[15 + inpos] >> 16) | ((input[16 + inpos] & 32767) << (31 - 15));
    output[17 + outpos] = (input[16 + inpos] >> 15) | ((input[17 + inpos] & 16383) << (31 - 14));
    output[18 + outpos] = (input[17 + inpos] >> 14) | ((input[18 + inpos] & 8191) << (31 - 13));
    output[19 + outpos] = (input[18 + inpos] >> 13) | ((input[19 + inpos] & 4095) << (31 - 12));
    output[20 + outpos] = (input[19 + inpos] >> 12) | ((input[20 + inpos] & 2047) << (31 - 11));
    output[21 + outpos] = (input[20 + inpos] >> 11) | ((input[21 + inpos] & 1023) << (31 - 10));
    output[22 + outpos] = (input[21 + inpos] >> 10) | ((input[22 + inpos] & 511) << (31 - 9));
    output[23 + outpos] = (input[22 + inpos] >> 9) | ((input[23 + inpos] & 255) << (31 - 8));
    output[24 + outpos] = (input[23 + inpos] >> 8) | ((input[24 + inpos] & 127) << (31 - 7));
    output[25 + outpos] = (input[24 + inpos] >> 7) | ((input[25 + inpos] & 63) << (31 - 6));
    output[26 + outpos] = (input[25 + inpos] >> 6) | ((input[26 + inpos] & 31) << (31 - 5));
    output[27 + outpos] = (input[26 + inpos] >> 5) | ((input[27 + inpos] & 15) << (31 - 4));
    output[28 + outpos] = (input[27 + inpos] >> 4) | ((input[28 + inpos] & 7) << (31 - 3));
    output[29 + outpos] = (input[28 + inpos] >> 3) | ((input[29 + inpos] & 3) << (31 - 2));
    output[30 + outpos] = (input[29 + inpos] >> 2) | ((input[30 + inpos] & 1) << (31 - 1));
    output[31 + outpos] = input[30 + inpos] >> 1;
}

// Java
// protected static void fastunpack32(final int[] in, int inpos,
//         final int[] out, int outpos) {
//         System.arraycopy(in, inpos, out, outpos, 32);
// }

fn fast_unpack32(input: &mut [i32], inpos: usize, output: &mut [i32], outpos: usize) {}
