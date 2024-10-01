pub fn fast_unpack(input: &mut [i32], inpos: isize, output: &mut [i32], outpos: isize, bit: isize) {
    match bit {
        0 => fast_unpack0(output, outpos),
        1 => fast_unpack1(input, inpos, output, outpos),
        2 => fast_unpack2(input, inpos, output, outpos),
        3 => fast_unpack3(input, inpos, output, outpos),
        4 => fast_unpack4(input, inpos, output, outpos),
        5 => fast_unpack5(input, inpos, output, outpos),
        6 => fast_unpack6(input, inpos, output, outpos),
        // 7 => fast_unpack7(input, inpos, output, outpos),
        // 8 => fast_unpack8(input, inpos, output, outpos),
        // 9 => fast_unpack9(input, inpos, output, outpos),
        // 10 => fast_unpack10(input, inpos, output, outpos),
        // 11 => fast_unpack11(input, inpos, output, outpos),
        // 12 => fast_unpack12(input, inpos, output, outpos),
        // 13 => fast_unpack13(input, inpos, output, outpos),
        // 14 => fast_unpack14(input, inpos, output, outpos),
        // 15 => fast_unpack15(input, inpos, output, outpos),
        // 16 => fast_unpack16(input, inpos, output, outpos),
        // 17 => fast_unpack17(input, inpos, output, outpos),
        // 18 => fast_unpack18(input, inpos, output, outpos),
        // 19 => fast_unpack19(input, inpos, output, outpos),
        // 20 => fast_unpack20(input, inpos, output, outpos),
        // 21 => fast_unpack21(input, inpos, output, outpos),
        // 22 => fast_unpack22(input, inpos, output, outpos),
        // 23 => fast_unpack23(input, inpos, output, outpos),
        // 24 => fast_unpack24(input, inpos, output, outpos),
        // 25 => fast_unpack25(input, inpos, output, outpos),
        // 26 => fast_unpack26(input, inpos, output, outpos),
        // 27 => fast_unpack27(input, inpos, output, outpos),
        // 28 => fast_unpack28(input, inpos, output, outpos),
        // 29 => fast_unpack29(input, inpos, output, outpos),
        // 30 => fast_unpack30(input, inpos, output, outpos),
        // 31 => fast_unpack31(input, inpos, output, outpos),
        // 32 => fast_unpack32(input, inpos as usize, output, outpos as usize),
        _ => panic!("Unsupported bit width"),
    }
}

fn fast_unpack0(output: &mut [i32], outpos: isize) {
    for i in outpos..outpos + 32 {
        output[i as usize] = 0;
    }
}

fn fast_unpack1(input: &mut [i32], inpos: isize, output: &mut [i32], outpos: isize) {
    output[0 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 0) as i32 & 1;
    output[1 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 1) as i32 & 1;
    output[2 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 2) as i32 & 1;
    output[3 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 3) as i32 & 1;
    output[4 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 4) as i32 & 1;
    output[5 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 5) as i32 & 1;
    output[6 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 6) as i32 & 1;
    output[7 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 7) as i32 & 1;
    output[8 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 8) as i32 & 1;
    output[9 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 9) as i32 & 1;
    output[10 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 10) as i32 & 1;
    output[11 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 11) as i32 & 1;
    output[12 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 12) as i32 & 1;
    output[13 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 13) as i32 & 1;
    output[14 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 14) as i32 & 1;
    output[15 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 15) as i32 & 1;
    output[16 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 16) as i32 & 1;
    output[17 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 17) as i32 & 1;
    output[18 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 18) as i32 & 1;
    output[19 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 19) as i32 & 1;
    output[20 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 20) as i32 & 1;
    output[21 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 21) as i32 & 1;
    output[22 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 22) as i32 & 1;
    output[23 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 23) as i32 & 1;
    output[24 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 24) as i32 & 1;
    output[25 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 25) as i32 & 1;
    output[26 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 26) as i32 & 1;
    output[27 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 27) as i32 & 1;
    output[28 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 28) as i32 & 1;
    output[29 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 29) as i32 & 1;
    output[30 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 30) as i32 & 1;
    output[31 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 31) as i32;
}

fn fast_unpack2(input: &mut [i32], inpos: isize, output: &mut [i32], outpos: isize) {
    output[0 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 0) as i32 & 3;
    output[1 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 2) as i32 & 3;
    output[2 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 4) as i32 & 3;
    output[3 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 6) as i32 & 3;
    output[4 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 8) as i32 & 3;
    output[5 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 10) as i32 & 3;
    output[6 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 12) as i32 & 3;
    output[7 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 14) as i32 & 3;
    output[8 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 16) as i32 & 3;
    output[9 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 18) as i32 & 3;
    output[10 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 20) as i32 & 3;
    output[11 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 22) as i32 & 3;
    output[12 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 24) as i32 & 3;
    output[13 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 26) as i32 & 3;
    output[14 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 28) as i32 & 3;
    output[15 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 30) as i32;
    output[16 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 0) as i32 & 3;
    output[17 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 2) as i32 & 3;
    output[18 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 4) as i32 & 3;
    output[19 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 6) as i32 & 3;
    output[20 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 8) as i32 & 3;
    output[21 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 10) as i32 & 3;
    output[22 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 12) as i32 & 3;
    output[23 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 14) as i32 & 3;
    output[24 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 16) as i32 & 3;
    output[25 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 18) as i32 & 3;
    output[26 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 20) as i32 & 3;
    output[27 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 22) as i32 & 3;
    output[28 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 24) as i32 & 3;
    output[29 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 26) as i32 & 3;
    output[30 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 28) as i32 & 3;
    output[31 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 30) as i32;
}

fn fast_unpack3(input: &mut [i32], inpos: isize, output: &mut [i32], outpos: isize) {
    output[0 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 0) as i32 & 7;
    output[1 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 3) as i32 & 7;
    output[2 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 6) as i32 & 7;
    output[3 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 9) as i32 & 7;
    output[4 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 12) as i32 & 7;
    output[5 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 15) as i32 & 7;
    output[6 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 18) as i32 & 7;
    output[7 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 21) as i32 & 7;
    output[8 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 24) as i32 & 7;
    output[9 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 27) as i32 & 7;
    output[10 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 30) as i32
        | ((input[1 + inpos as usize] & 1) << (3 - 1));
    output[11 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 1) as i32 & 7;
    output[12 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 4) as i32 & 7;
    output[13 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 7) as i32 & 7;
    output[14 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 10) as i32 & 7;
    output[15 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 13) as i32 & 7;
    output[16 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 16) as i32 & 7;
    output[17 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 19) as i32 & 7;
    output[18 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 22) as i32 & 7;
    output[19 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 25) as i32 & 7;
    output[20 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 28) as i32 & 7;
    output[21 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 31) as i32
        | ((input[2 + inpos as usize] & 3) << (3 - 2));
    output[22 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 2) as i32 & 7;
    output[23 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 5) as i32 & 7;
    output[24 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 8) as i32 & 7;
    output[25 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 11) as i32 & 7;
    output[26 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 14) as i32 & 7;
    output[27 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 17) as i32 & 7;
    output[28 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 20) as i32 & 7;
    output[29 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 23) as i32 & 7;
    output[30 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 26) as i32 & 7;
    output[31 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 29) as i32;
}

fn fast_unpack4(input: &mut [i32], inpos: isize, output: &mut [i32], outpos: isize) {
    output[0 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 0) as i32 & 15;
    output[1 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 4) as i32 & 15;
    output[2 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 8) as i32 & 15;
    output[3 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 12) as i32 & 15;
    output[4 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 16) as i32 & 15;
    output[5 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 20) as i32 & 15;
    output[6 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 24) as i32 & 15;
    output[7 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 28) as i32;
    output[8 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 0) as i32 & 15;
    output[9 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 4) as i32 & 15;
    output[10 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 8) as i32 & 15;
    output[11 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 12) as i32 & 15;
    output[12 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 16) as i32 & 15;
    output[13 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 20) as i32 & 15;
    output[14 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 24) as i32 & 15;
    output[15 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 28) as i32;
    output[16 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 0) as i32 & 15;
    output[17 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 4) as i32 & 15;
    output[18 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 8) as i32 & 15;
    output[19 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 12) as i32 & 15;
    output[20 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 16) as i32 & 15;
    output[21 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 20) as i32 & 15;
    output[22 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 24) as i32 & 15;
    output[23 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 28) as i32;
    output[24 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 0) as i32 & 15;
    output[25 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 4) as i32 & 15;
    output[26 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 8) as i32 & 15;
    output[27 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 12) as i32 & 15;
    output[28 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 16) as i32 & 15;
    output[29 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 20) as i32 & 15;
    output[30 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 24) as i32 & 15;
    output[31 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 28) as i32;
}

fn fast_unpack5(input: &mut [i32], inpos: isize, output: &mut [i32], outpos: isize) {
    output[0 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 0) as i32 & 31;
    output[1 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 5) as i32 & 31;
    output[2 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 10) as i32 & 31;
    output[3 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 15) as i32 & 31;
    output[4 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 20) as i32 & 31;
    output[5 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 25) as i32 & 31;
    output[6 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 30) as i32
        | ((input[1 + inpos as usize] & 7) << (5 - 3));
    output[7 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 3) as i32 & 31;
    output[8 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 8) as i32 & 31;
    output[9 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 13) as i32 & 31;
    output[10 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 18) as i32 & 31;
    output[11 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 23) as i32 & 31;
    output[12 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 28) as i32
        | ((input[2 + inpos as usize] & 1) << (5 - 1));
    output[13 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 1) as i32 & 31;
    output[14 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 6) as i32 & 31;
    output[15 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 11) as i32 & 31;
    output[16 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 16) as i32 & 31;
    output[17 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 21) as i32 & 31;
    output[18 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 26) as i32 & 31;
    output[19 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 31) as i32
        | ((input[3 + inpos as usize] & 15) << (5 - 4));
    output[20 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 4) as i32 & 31;
    output[21 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 9) as i32 & 31;
    output[22 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 14) as i32 & 31;
    output[23 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 19) as i32 & 31;
    output[24 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 24) as i32 & 31;
    output[25 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 29) as i32
        | ((input[4 + inpos as usize] & 3) << (5 - 2));
    output[26 + outpos as usize] = ((input[4 + inpos as usize]) as u32 >> 2) as i32 & 31;
    output[27 + outpos as usize] = ((input[4 + inpos as usize]) as u32 >> 7) as i32 & 31;
    output[28 + outpos as usize] = ((input[4 + inpos as usize]) as u32 >> 12) as i32 & 31;
    output[29 + outpos as usize] = ((input[4 + inpos as usize]) as u32 >> 17) as i32 & 31;
    output[30 + outpos as usize] = ((input[4 + inpos as usize]) as u32 >> 22) as i32 & 31;
    output[31 + outpos as usize] = ((input[4 + inpos as usize]) as u32 >> 27) as i32;
}

fn fast_unpack6(input: &mut [i32], inpos: isize, output: &mut [i32], outpos: isize) {
    output[0 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 0) as i32 & 63;
    output[1 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 6) as i32 & 63;
    output[2 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 12) as i32 & 63;
    output[3 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 18) as i32 & 63;
    output[4 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 24) as i32 & 63;
    output[5 + outpos as usize] = ((input[0 + inpos as usize]) as u32 >> 30) as i32
        | ((input[1 + inpos as usize] & 15) << (6 - 4));
    output[6 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 4) as i32 & 63;
    output[7 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 10) as i32 & 63;
    output[8 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 16) as i32 & 63;
    output[9 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 22) as i32 & 63;
    output[10 + outpos as usize] = ((input[1 + inpos as usize]) as u32 >> 28) as i32
        | ((input[2 + inpos as usize] & 3) << (6 - 2));
    output[11 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 2) as i32 & 63;
    output[12 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 8) as i32 & 63;
    output[13 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 14) as i32 & 63;
    output[14 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 20) as i32 & 63;
    output[15 + outpos as usize] = ((input[2 + inpos as usize]) as u32 >> 26) as i32;
    output[16 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 0) as i32 & 63;
    output[17 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 6) as i32 & 63;
    output[18 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 12) as i32 & 63;
    output[19 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 18) as i32 & 63;
    output[20 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 24) as i32 & 63;
    output[21 + outpos as usize] = ((input[3 + inpos as usize]) as u32 >> 30) as i32
        | ((input[4 + inpos as usize] & 15) << (6 - 4));
    output[22 + outpos as usize] = ((input[4 + inpos as usize]) as u32 >> 4) as i32 & 63;
    output[23 + outpos as usize] = ((input[4 + inpos as usize]) as u32 >> 10) as i32 & 63;
    output[24 + outpos as usize] = ((input[4 + inpos as usize]) as u32 >> 16) as i32 & 63;
    output[25 + outpos as usize] = ((input[4 + inpos as usize]) as u32 >> 22) as i32 & 63;
    output[26 + outpos as usize] = ((input[4 + inpos as usize]) as u32 >> 28) as i32
        | ((input[5 + inpos as usize] & 3) << (6 - 2));
    output[27 + outpos as usize] = ((input[5 + inpos as usize]) as u32 >> 2) as i32 & 63;
    output[28 + outpos as usize] = ((input[5 + inpos as usize]) as u32 >> 8) as i32 & 63;
    output[29 + outpos as usize] = ((input[5 + inpos as usize]) as u32 >> 14) as i32 & 63;
    output[30 + outpos as usize] = ((input[5 + inpos as usize]) as u32 >> 20) as i32 & 63;
    output[31 + outpos as usize] = ((input[5 + inpos as usize]) as u32 >> 26) as i32;
}

// fn fast_unpack7(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 127;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 7) & 127;
//     output[2 + outpos as usize] = (input[0 + inpos as usize] >> 14) & 127;
//     output[3 + outpos as usize] = (input[0 + inpos as usize] >> 21) & 127;
//     output[4 + outpos as usize] = (input[0 + inpos as usize] >> 28) & 127;
//     output[5 + outpos as usize] =
//         input[1 + inpos as usize] >> 0 | (input[2 + inpos as usize] & 3) << (7 - 2);
//     output[6 + outpos as usize] = (input[2 + inpos as usize] >> 2) & 127;
//     output[7 + outpos as usize] = (input[2 + inpos as usize] >> 9) & 127;
//     output[8 + outpos as usize] = (input[2 + inpos as usize] >> 16) & 127;
//     output[9 + outpos as usize] = (input[2 + inpos as usize] >> 23) & 127;
//     output[10 + outpos as usize] = (input[2 + inpos as usize] >> 30) & 127;
//     output[11 + outpos as usize] =
//         input[3 + inpos as usize] >> 0 | (input[4 + inpos as usize] & 3) << (7 - 2);
//     output[12 + outpos as usize] = (input[4 + inpos as usize] >> 2) & 127;
//     output[13 + outpos as usize] = (input[4 + inpos as usize] >> 9) & 127;
//     output[14 + outpos as usize] = (input[4 + inpos as usize] >> 16) & 127;
//     output[15 + outpos as usize] = (input[4 + inpos as usize] >> 23) & 127;
//     output[16 + outpos as usize] = (input[4 + inpos as usize] >> 30) & 127;
//     output[17 + outpos as usize] =
//         input[5 + inpos as usize] >> 0 | (input[6 + inpos as usize] & 3) << (7 - 2);
//     output[18 + outpos as usize] = (input[6 + inpos as usize] >> 2) & 127;
//     output[19 + outpos as usize] = (input[6 + inpos as usize] >> 9) & 127;
//     output[20 + outpos as usize] = (input[6 + inpos as usize] >> 16) & 127;
//     output[21 + outpos as usize] = (input[6 + inpos as usize] >> 23) & 127;
//     output[22 + outpos as usize] = (input[6 + inpos as usize] >> 30) & 127;
//     output[23 + outpos as usize] =
//         input[7 + inpos as usize] >> 0 | (input[8 + inpos as usize] & 3) << (7 - 2);
//     output[24 + outpos as usize] = (input[8 + inpos as usize] >> 2) & 127;
//     output[25 + outpos as usize] = (input[8 + inpos as usize] >> 9) & 127;
//     output[26 + outpos as usize] = (input[8 + inpos as usize] >> 16) & 127;
//     output[27 + outpos as usize] = (input[8 + inpos as usize] >> 23) & 127;
//     output[28 + outpos as usize] = (input[8 + inpos as usize] >> 30) & 127;
//     output[29 + outpos as usize] =
//         input[9 + inpos as usize] >> 0 | (input[10 + inpos as usize] & 3) << (7 - 2);
//     output[30 + outpos as usize] = (input[10 + inpos as usize] >> 2) & 127;
//     output[31 + outpos as usize] = input[10 + inpos as usize] >> 9;
// }
//
// fn fast_unpack8(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 255;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 8) & 255;
//     output[2 + outpos as usize] = (input[0 + inpos as usize] >> 16) & 255;
//     output[3 + outpos as usize] = (input[0 + inpos as usize] >> 24) & 255;
//     output[4 + outpos as usize] = input[1 + inpos as usize] & 255;
//     output[5 + outpos as usize] = (input[1 + inpos as usize] >> 8) & 255;
//     output[6 + outpos as usize] = (input[1 + inpos as usize] >> 16) & 255;
//     output[7 + outpos as usize] = (input[1 + inpos as usize] >> 24) & 255;
//     output[8 + outpos as usize] = input[2 + inpos as usize] & 255;
//     output[9 + outpos as usize] = (input[2 + inpos as usize] >> 8) & 255;
//     output[10 + outpos as usize] = (input[2 + inpos as usize] >> 16) & 255;
//     output[11 + outpos as usize] = (input[2 + inpos as usize] >> 24) & 255;
//     output[12 + outpos as usize] = input[3 + inpos as usize] & 255;
//     output[13 + outpos as usize] = (input[3 + inpos as usize] >> 8) & 255;
//     output[14 + outpos as usize] = (input[3 + inpos as usize] >> 16) & 255;
//     output[15 + outpos as usize] = (input[3 + inpos as usize] >> 24) & 255;
//     output[16 + outpos as usize] = input[4 + inpos as usize] & 255;
//     output[17 + outpos as usize] = (input[4 + inpos as usize] >> 8) & 255;
//     output[18 + outpos as usize] = (input[4 + inpos as usize] >> 16) & 255;
//     output[19 + outpos as usize] = (input[4 + inpos as usize] >> 24) & 255;
//     output[20 + outpos as usize] = input[5 + inpos as usize] & 255;
//     output[21 + outpos as usize] = (input[5 + inpos as usize] >> 8) & 255;
//     output[22 + outpos as usize] = (input[5 + inpos as usize] >> 16) & 255;
//     output[23 + outpos as usize] = (input[5 + inpos as usize] >> 24) & 255;
//     output[24 + outpos as usize] = input[6 + inpos as usize] & 255;
//     output[25 + outpos as usize] = (input[6 + inpos as usize] >> 8) & 255;
//     output[26 + outpos as usize] = (input[6 + inpos as usize] >> 16) & 255;
//     output[27 + outpos as usize] = (input[6 + inpos as usize] >> 24) & 255;
//     output[28 + outpos as usize] = input[7 + inpos as usize] & 255;
//     output[29 + outpos as usize] = (input[7 + inpos as usize] >> 8) & 255;
//     output[30 + outpos as usize] = (input[7 + inpos as usize] >> 16) & 255;
//     output[31 + outpos as usize] = input[7 + inpos as usize] >> 24;
// }
//
// fn fast_unpack9(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 511;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 9) & 511;
//     output[2 + outpos as usize] = (input[0 + inpos as usize] >> 18) & 511;
//     output[3 + outpos as usize] = (input[0 + inpos as usize] >> 27) & 511;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 0 | (input[2 + inpos as usize] & 127) << (9 - 7);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 7) & 511;
//     output[6 + outpos as usize] = (input[2 + inpos as usize] >> 16) & 511;
//     output[7 + outpos as usize] = (input[2 + inpos as usize] >> 25) & 511;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 0 | (input[4 + inpos as usize] & 127) << (9 - 7);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 7) & 511;
//     output[10 + outpos as usize] = (input[4 + inpos as usize] >> 16) & 511;
//     output[11 + outpos as usize] = (input[4 + inpos as usize] >> 25) & 511;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 0 | (input[6 + inpos as usize] & 127) << (9 - 7);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 7) & 511;
//     output[14 + outpos as usize] = (input[6 + inpos as usize] >> 16) & 511;
//     output[15 + outpos as usize] = (input[6 + inpos as usize] >> 25) & 511;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 0 | (input[8 + inpos as usize] & 127) << (9 - 7);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 7) & 511;
//     output[18 + outpos as usize] = (input[8 + inpos as usize] >> 16) & 511;
//     output[19 + outpos as usize] = (input[8 + inpos as usize] >> 25) & 511;
//     output[20 + outpos as usize] =
//         input[9 + inpos as usize] >> 0 | (input[10 + inpos as usize] & 127) << (9 - 7);
//     output[21 + outpos as usize] = (input[10 + inpos as usize] >> 7) & 511;
//     output[22 + outpos as usize] = (input[10 + inpos as usize] >> 16) & 511;
//     output[23 + outpos as usize] = (input[10 + inpos as usize] >> 25) & 511;
//     output[24 + outpos as usize] =
//         input[11 + inpos as usize] >> 0 | (input[12 + inpos as usize] & 127) << (9 - 7);
//     output[25 + outpos as usize] = (input[12 + inpos as usize] >> 7) & 511;
//     output[26 + outpos as usize] = (input[12 + inpos as usize] >> 16) & 511;
//     output[27 + outpos as usize] = (input[12 + inpos as usize] >> 25) & 511;
//     output[28 + outpos as usize] =
//         input[13 + inpos as usize] >> 0 | (input[14 + inpos as usize] & 127) << (9 - 7);
//     output[29 + outpos as usize] = (input[14 + inpos as usize] >> 7) & 511;
//     output[30 + outpos as usize] = (input[14 + inpos as usize] >> 16) & 511;
//     output[31 + outpos as usize] = input[14 + inpos as usize] >> 25;
// }
//
// fn fast_unpack10(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 1023;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 10) & 1023;
//     output[2 + outpos as usize] = (input[0 + inpos as usize] >> 20) & 1023;
//     output[3 + outpos as usize] =
//         input[0 + inpos as usize] >> 30 | (input[1 + inpos as usize] & 255) << (10 - 8);
//     output[4 + outpos as usize] = (input[1 + inpos as usize] >> 8) & 1023;
//     output[5 + outpos as usize] = (input[1 + inpos as usize] >> 18) & 1023;
//     output[6 + outpos as usize] =
//         input[1 + inpos as usize] >> 28 | (input[2 + inpos as usize] & 63) << (10 - 6);
//     output[7 + outpos as usize] = (input[2 + inpos as usize] >> 6) & 1023;
//     output[8 + outpos as usize] = (input[2 + inpos as usize] >> 16) & 1023;
//     output[9 + outpos as usize] =
//         input[2 + inpos as usize] >> 26 | (input[3 + inpos as usize] & 15) << (10 - 4);
//     output[10 + outpos as usize] = (input[3 + inpos as usize] >> 4) & 1023;
//     output[11 + outpos as usize] = (input[3 + inpos as usize] >> 14) & 1023;
//     output[12 + outpos as usize] =
//         input[3 + inpos as usize] >> 24 | (input[4 + inpos as usize] & 3) << (10 - 2);
//     output[13 + outpos as usize] = (input[4 + inpos as usize] >> 2) & 1023;
//     output[14 + outpos as usize] = (input[4 + inpos as usize] >> 12) & 1023;
//     output[15 + outpos as usize] = input[4 + inpos as usize] >> 22;
//     output[16 + outpos as usize] = (input[5 + inpos as usize] >> 0) & 1023;
//     output[17 + outpos as usize] = (input[5 + inpos as usize] >> 10) & 1023;
//     output[18 + outpos as usize] = (input[5 + inpos as usize] >> 20) & 1023;
//     output[19 + outpos as usize] =
//         input[5 + inpos as usize] >> 30 | (input[6 + inpos as usize] & 255) << (10 - 8);
//     output[20 + outpos as usize] = (input[6 + inpos as usize] >> 8) & 1023;
//     output[21 + outpos as usize] = (input[6 + inpos as usize] >> 18) & 1023;
//     output[22 + outpos as usize] =
//         input[6 + inpos as usize] >> 28 | (input[7 + inpos as usize] & 63) << (10 - 6);
//     output[23 + outpos as usize] = (input[7 + inpos as usize] >> 6) & 1023;
//     output[24 + outpos as usize] = (input[7 + inpos as usize] >> 16) & 1023;
//     output[25 + outpos as usize] =
//         input[7 + inpos as usize] >> 26 | (input[8 + inpos as usize] & 15) << (10 - 4);
//     output[26 + outpos as usize] = (input[8 + inpos as usize] >> 4) & 1023;
//     output[27 + outpos as usize] = (input[8 + inpos as usize] >> 14) & 1023;
//     output[28 + outpos as usize] =
//         input[8 + inpos as usize] >> 24 | (input[9 + inpos as usize] & 3) << (10 - 2);
//     output[29 + outpos as usize] = (input[9 + inpos as usize] >> 2) & 1023;
//     output[30 + outpos as usize] = (input[9 + inpos as usize] >> 12) & 1023;
//     output[31 + outpos as usize] = input[9 + inpos as usize] >> 22;
// }
//
// fn fast_unpack11(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 2047;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 11) & 2047;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 22 | (input[1 + inpos as usize] & 255) << (11 - 8);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 8) & 2047;
//     output[4 + outpos as usize] = (input[1 + inpos as usize] >> 19) & 2047;
//     output[5 + outpos as usize] =
//         input[1 + inpos as usize] >> 30 | (input[2 + inpos as usize] & 127) << (11 - 7);
//     output[6 + outpos as usize] = (input[2 + inpos as usize] >> 7) & 2047;
//     output[7 + outpos as usize] =
//         input[2 + inpos as usize] >> 18 | (input[3 + inpos as usize] & 15) << (11 - 4);
//     output[8 + outpos as usize] = (input[3 + inpos as usize] >> 4) & 2047;
//     output[9 + outpos as usize] = (input[3 + inpos as usize] >> 15) & 2047;
//     output[10 + outpos as usize] =
//         input[3 + inpos as usize] >> 26 | (input[4 + inpos as usize] & 63) << (11 - 6);
//     output[11 + outpos as usize] = (input[4 + inpos as usize] >> 6) & 2047;
//     output[12 + outpos as usize] =
//         input[4 + inpos as usize] >> 17 | (input[5 + inpos as usize] & 7) << (11 - 3);
//     output[13 + outpos as usize] = (input[5 + inpos as usize] >> 3) & 2047;
//     output[14 + outpos as usize] = (input[5 + inpos as usize] >> 14) & 2047;
//     output[15 + outpos as usize] =
//         input[5 + inpos as usize] >> 25 | (input[6 + inpos as usize] & 31) << (11 - 5);
//     output[16 + outpos as usize] = (input[6 + inpos as usize] >> 5) & 2047;
//     output[17 + outpos as usize] =
//         input[6 + inpos as usize] >> 16 | (input[7 + inpos as usize] & 3) << (11 - 2);
//     output[18 + outpos as usize] = (input[7 + inpos as usize] >> 2) & 2047;
//     output[19 + outpos as usize] = (input[7 + inpos as usize] >> 13) & 2047;
//     output[20 + outpos as usize] =
//         input[7 + inpos as usize] >> 24 | (input[8 + inpos as usize] & 127) << (11 - 7);
//     output[21 + outpos as usize] = (input[8 + inpos as usize] >> 7) & 2047;
//     output[22 + outpos as usize] =
//         input[8 + inpos as usize] >> 18 | (input[9 + inpos as usize] & 15) << (11 - 4);
//     output[23 + outpos as usize] = (input[9 + inpos as usize] >> 4) & 2047;
//     output[24 + outpos as usize] = (input[9 + inpos as usize] >> 15) & 2047;
//     output[25 + outpos as usize] =
//         input[9 + inpos as usize] >> 26 | (input[10 + inpos as usize] & 63) << (11 - 6);
//     output[26 + outpos as usize] = (input[10 + inpos as usize] >> 6) & 2047;
//     output[27 + outpos as usize] =
//         input[10 + inpos as usize] >> 17 | (input[11 + inpos as usize] & 7) << (11 - 3);
//     output[28 + outpos as usize] = (input[11 + inpos as usize] >> 3) & 2047;
//     output[29 + outpos as usize] = (input[11 + inpos as usize] >> 14) & 2047;
//     output[30 + outpos as usize] =
//         input[11 + inpos as usize] >> 25 | (input[12 + inpos as usize] & 31) << (11 - 5);
//     output[31 + outpos as usize] = (input[12 + inpos as usize] >> 5) & 2047;
// }
//
// fn fast_unpack12(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 4095;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 12) & 4095;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 24 | (input[1 + inpos as usize] & 15) << (12 - 4);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 4) & 4095;
//     output[4 + outpos as usize] = (input[1 + inpos as usize] >> 16) & 4095;
//     output[5 + outpos as usize] =
//         input[1 + inpos as usize] >> 28 | (input[2 + inpos as usize] & 255) << (12 - 8);
//     output[6 + outpos as usize] = (input[2 + inpos as usize] >> 8) & 4095;
//     output[7 + outpos as usize] =
//         input[2 + inpos as usize] >> 20 | (input[3 + inpos as usize] & 3) << (12 - 2);
//     output[8 + outpos as usize] = (input[3 + inpos as usize] >> 2) & 4095;
//     output[9 + outpos as usize] = (input[3 + inpos as usize] >> 14) & 4095;
//     output[10 + outpos as usize] =
//         input[3 + inpos as usize] >> 26 | (input[4 + inpos as usize] & 63) << (12 - 6);
//     output[11 + outpos as usize] = (input[4 + inpos as usize] >> 6) & 4095;
//     output[12 + outpos as usize] = (input[4 + inpos as usize] >> 18) & 4095;
//     output[13 + outpos as usize] =
//         input[4 + inpos as usize] >> 30 | (input[5 + inpos as usize] & 1023) << (12 - 10);
//     output[14 + outpos as usize] = (input[5 + inpos as usize] >> 10) & 4095;
//     output[15 + outpos as usize] = input[5 + inpos as usize] >> 22;
//     output[16 + outpos as usize] = (input[6 + inpos as usize] >> 0) & 4095;
//     output[17 + outpos as usize] = (input[6 + inpos as usize] >> 12) & 4095;
//     output[18 + outpos as usize] =
//         input[6 + inpos as usize] >> 24 | (input[7 + inpos as usize] & 15) << (12 - 4);
//     output[19 + outpos as usize] = (input[7 + inpos as usize] >> 4) & 4095;
//     output[20 + outpos as usize] = (input[7 + inpos as usize] >> 16) & 4095;
//     output[21 + outpos as usize] =
//         input[7 + inpos as usize] >> 28 | (input[8 + inpos as usize] & 255) << (12 - 8);
//     output[22 + outpos as usize] = (input[8 + inpos as usize] >> 8) & 4095;
//     output[23 + outpos as usize] =
//         input[8 + inpos as usize] >> 20 | (input[9 + inpos as usize] & 3) << (12 - 2);
//     output[24 + outpos as usize] = (input[9 + inpos as usize] >> 2) & 4095;
//     output[25 + outpos as usize] = (input[9 + inpos as usize] >> 14) & 4095;
//     output[26 + outpos as usize] =
//         input[9 + inpos as usize] >> 26 | (input[10 + inpos as usize] & 63) << (12 - 6);
//     output[27 + outpos as usize] = (input[10 + inpos as usize] >> 6) & 4095;
//     output[28 + outpos as usize] = (input[10 + inpos as usize] >> 18) & 4095;
//     output[29 + outpos as usize] =
//         input[10 + inpos as usize] >> 30 | (input[11 + inpos as usize] & 1023) << (12 - 10);
//     output[30 + outpos as usize] = (input[11 + inpos as usize] >> 10) & 4095;
//     output[31 + outpos as usize] = input[11 + inpos as usize] >> 22;
// }
//
// fn fast_unpack13(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 8191;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 13) & 8191;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 26 | (input[1 + inpos as usize] & 127) << (13 - 7);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 7) & 8191;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 20 | (input[2 + inpos as usize] & 3) << (13 - 2);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 2) & 8191;
//     output[6 + outpos as usize] = (input[2 + inpos as usize] >> 15) & 8191;
//     output[7 + outpos as usize] =
//         input[2 + inpos as usize] >> 28 | (input[3 + inpos as usize] & 255) << (13 - 8);
//     output[8 + outpos as usize] = (input[3 + inpos as usize] >> 8) & 8191;
//     output[9 + outpos as usize] =
//         input[3 + inpos as usize] >> 21 | (input[4 + inpos as usize] & 7) << (13 - 3);
//     output[10 + outpos as usize] = (input[4 + inpos as usize] >> 3) & 8191;
//     output[11 + outpos as usize] = (input[4 + inpos as usize] >> 16) & 8191;
//     output[12 + outpos as usize] =
//         input[4 + inpos as usize] >> 29 | (input[5 + inpos as usize] & 511) << (13 - 9);
//     output[13 + outpos as usize] = (input[5 + inpos as usize] >> 9) & 8191;
//     output[14 + outpos as usize] = input[5 + inpos as usize] >> 22;
//     output[15 + outpos as usize] = (input[6 + inpos as usize] >> 0) & 8191;
//     output[16 + outpos as usize] = (input[6 + inpos as usize] >> 13) & 8191;
//     output[17 + outpos as usize] =
//         input[6 + inpos as usize] >> 26 | (input[7 + inpos as usize] & 127) << (13 - 7);
//     output[18 + outpos as usize] = (input[7 + inpos as usize] >> 7) & 8191;
//     output[19 + outpos as usize] =
//         input[7 + inpos as usize] >> 20 | (input[8 + inpos as usize] & 3) << (13 - 2);
//     output[20 + outpos as usize] = (input[8 + inpos as usize] >> 2) & 8191;
//     output[21 + outpos as usize] = (input[8 + inpos as usize] >> 15) & 8191;
//     output[22 + outpos as usize] =
//         input[8 + inpos as usize] >> 28 | (input[9 + inpos as usize] & 255) << (13 - 8);
//     output[23 + outpos as usize] = (input[9 + inpos as usize] >> 8) & 8191;
//     output[24 + outpos as usize] =
//         input[9 + inpos as usize] >> 21 | (input[10 + inpos as usize] & 7) << (13 - 3);
//     output[25 + outpos as usize] = (input[10 + inpos as usize] >> 3) & 8191;
//     output[26 + outpos as usize] = (input[10 + inpos as usize] >> 16) & 8191;
//     output[27 + outpos as usize] =
//         input[10 + inpos as usize] >> 29 | (input[11 + inpos as usize] & 511) << (13 - 9);
//     output[28 + outpos as usize] = (input[11 + inpos as usize] >> 9) & 8191;
//     output[29 + outpos as usize] = input[11 + inpos as usize] >> 22;
//     output[30 + outpos as usize] = (input[12 + inpos as usize] >> 0) & 8191;
//     output[31 + outpos as usize] = (input[12 + inpos as usize] >> 13) & 8191;
// }
//
// fn fast_unpack14(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 16383;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 14) & 16383;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 28 | (input[1 + inpos as usize] & 1023) << (14 - 10);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 10) & 16383;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 24 | (input[2 + inpos as usize] & 63) << (14 - 6);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 6) & 16383;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 20 | (input[3 + inpos as usize] & 3) << (14 - 2);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 2) & 16383;
//     output[8 + outpos as usize] = (input[3 + inpos as usize] >> 16) & 16383;
//     output[9 + outpos as usize] =
//         input[3 + inpos as usize] >> 30 | (input[4 + inpos as usize] & 4095) << (14 - 12);
//     output[10 + outpos as usize] = (input[4 + inpos as usize] >> 12) & 16383;
//     output[11 + outpos as usize] =
//         input[4 + inpos as usize] >> 26 | (input[5 + inpos as usize] & 255) << (14 - 8);
//     output[12 + outpos as usize] = (input[5 + inpos as usize] >> 8) & 16383;
//     output[13 + outpos as usize] = input[5 + inpos as usize] >> 22;
//     output[14 + outpos as usize] = (input[6 + inpos as usize] >> 0) & 16383;
//     output[15 + outpos as usize] = (input[6 + inpos as usize] >> 14) & 16383;
//     output[16 + outpos as usize] =
//         input[6 + inpos as usize] >> 28 | (input[7 + inpos as usize] & 1023) << (14 - 10);
//     output[17 + outpos as usize] = (input[7 + inpos as usize] >> 10) & 16383;
//     output[18 + outpos as usize] =
//         input[7 + inpos as usize] >> 24 | (input[8 + inpos as usize] & 63) << (14 - 6);
//     output[19 + outpos as usize] = (input[8 + inpos as usize] >> 6) & 16383;
//     output[20 + outpos as usize] =
//         input[8 + inpos as usize] >> 20 | (input[9 + inpos as usize] & 3) << (14 - 2);
//     output[21 + outpos as usize] = (input[9 + inpos as usize] >> 2) & 16383;
//     output[22 + outpos as usize] = (input[9 + inpos as usize] >> 16) & 16383;
//     output[23 + outpos as usize] =
//         input[9 + inpos as usize] >> 30 | (input[10 + inpos as usize] & 4095) << (14 - 12);
//     output[24 + outpos as usize] = (input[10 + inpos as usize] >> 12) & 16383;
//     output[25 + outpos as usize] =
//         input[10 + inpos as usize] >> 26 | (input[11 + inpos as usize] & 255) << (14 - 8);
//     output[26 + outpos as usize] = (input[11 + inpos as usize] >> 8) & 16383;
//     output[27 + outpos as usize] = input[11 + inpos as usize] >> 22;
//     output[28 + outpos as usize] = (input[12 + inpos as usize] >> 0) & 16383;
//     output[29 + outpos as usize] = (input[12 + inpos as usize] >> 14) & 16383;
//     output[30 + outpos as usize] =
//         input[12 + inpos as usize] >> 28 | (input[13 + inpos as usize] & 1023) << (14 - 10);
//     output[31 + outpos as usize] = (input[13 + inpos as usize] >> 10) & 16383;
// }
//
// fn fast_unpack15(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 32767;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 15) & 32767;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 30 | (input[1 + inpos as usize] & 8191) << (15 - 13);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 13) & 32767;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 28 | (input[2 + inpos as usize] & 4095) << (15 - 12);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 12) & 32767;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 27 | (input[3 + inpos as usize] & 2047) << (15 - 11);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 11) & 32767;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 26 | (input[4 + inpos as usize] & 1023) << (15 - 10);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 10) & 32767;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 25 | (input[5 + inpos as usize] & 511) << (15 - 9);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 9) & 32767;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 24 | (input[6 + inpos as usize] & 255) << (15 - 8);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 8) & 32767;
//     output[14 + outpos as usize] =
//         input[6 + inpos as usize] >> 23 | (input[7 + inpos as usize] & 127) << (15 - 7);
//     output[15 + outpos as usize] = (input[7 + inpos as usize] >> 7) & 32767;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 22 | (input[8 + inpos as usize] & 63) << (15 - 6);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 6) & 32767;
//     output[18 + outpos as usize] =
//         input[8 + inpos as usize] >> 21 | (input[9 + inpos as usize] & 31) << (15 - 5);
//     output[19 + outpos as usize] = (input[9 + inpos as usize] >> 5) & 32767;
//     output[20 + outpos as usize] =
//         input[9 + inpos as usize] >> 20 | (input[10 + inpos as usize] & 15) << (15 - 4);
//     output[21 + outpos as usize] = (input[10 + inpos as usize] >> 4) & 32767;
//     output[22 + outpos as usize] =
//         input[10 + inpos as usize] >> 19 | (input[11 + inpos as usize] & 7) << (15 - 3);
//     output[23 + outpos as usize] = (input[11 + inpos as usize] >> 3) & 32767;
//     output[24 + outpos as usize] =
//         input[11 + inpos as usize] >> 18 | (input[12 + inpos as usize] & 3) << (15 - 2);
//     output[25 + outpos as usize] = (input[12 + inpos as usize] >> 2) & 32767;
//     output[26 + outpos as usize] = (input[12 + inpos as usize] >> 17) & 32767;
//     output[27 + outpos as usize] =
//         input[12 + inpos as usize] >> 32 | (input[13 + inpos as usize] & 16383) << (15 - 14);
//     output[28 + outpos as usize] = (input[13 + inpos as usize] >> 14) & 32767;
//     output[29 + outpos as usize] =
//         input[13 + inpos as usize] >> 29 | (input[14 + inpos as usize] & 8191) << (15 - 13);
//     output[30 + outpos as usize] = (input[14 + inpos as usize] >> 13) & 32767;
//     output[31 + outpos as usize] =
//         input[14 + inpos as usize] >> 28 | (input[15 + inpos as usize] & 4095) << (15 - 12);
// }
//
// fn fast_unpack16(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 65535;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 16) & 65535;
//     output[2 + outpos as usize] = input[1 + inpos as usize] & 65535;
//     output[3 + outpos as usize] = input[1 + inpos as usize] >> 16;
//     output[4 + outpos as usize] = input[2 + inpos as usize] & 65535;
//     output[5 + outpos as usize] = input[2 + inpos as usize] >> 16;
//     output[6 + outpos as usize] = input[3 + inpos as usize] & 65535;
//     output[7 + outpos as usize] = input[3 + inpos as usize] >> 16;
//     output[8 + outpos as usize] = input[4 + inpos as usize] & 65535;
//     output[9 + outpos as usize] = input[4 + inpos as usize] >> 16;
//     output[10 + outpos as usize] = input[5 + inpos as usize] & 65535;
//     output[11 + outpos as usize] = input[5 + inpos as usize] >> 16;
//     output[12 + outpos as usize] = input[6 + inpos as usize] & 65535;
//     output[13 + outpos as usize] = input[6 + inpos as usize] >> 16;
//     output[14 + outpos as usize] = input[7 + inpos as usize] & 65535;
//     output[15 + outpos as usize] = input[7 + inpos as usize] >> 16;
//     output[16 + outpos as usize] = input[8 + inpos as usize] & 65535;
//     output[17 + outpos as usize] = input[8 + inpos as usize] >> 16;
//     output[18 + outpos as usize] = input[9 + inpos as usize] & 65535;
//     output[19 + outpos as usize] = input[9 + inpos as usize] >> 16;
//     output[20 + outpos as usize] = input[10 + inpos as usize] & 65535;
//     output[21 + outpos as usize] = input[10 + inpos as usize] >> 16;
//     output[22 + outpos as usize] = input[11 + inpos as usize] & 65535;
//     output[23 + outpos as usize] = input[11 + inpos as usize] >> 16;
//     output[24 + outpos as usize] = input[12 + inpos as usize] & 65535;
//     output[25 + outpos as usize] = input[12 + inpos as usize] >> 16;
//     output[26 + outpos as usize] = input[13 + inpos as usize] & 65535;
//     output[27 + outpos as usize] = input[13 + inpos as usize] >> 16;
//     output[28 + outpos as usize] = input[14 + inpos as usize] & 65535;
//     output[29 + outpos as usize] = input[14 + inpos as usize] >> 16;
//     output[30 + outpos as usize] = input[15 + inpos as usize] & 65535;
//     output[31 + outpos as usize] = input[15 + inpos as usize] >> 16;
// }
//
// fn fast_unpack17(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 131071;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 17) & 131071;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 34 | (input[1 + inpos as usize] & 8191) << (17 - 13);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 13) & 131071;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 30 | (input[2 + inpos as usize] & 4095) << (17 - 12);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 12) & 131071;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 29 | (input[3 + inpos as usize] & 2047) << (17 - 11);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 11) & 131071;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 28 | (input[4 + inpos as usize] & 1023) << (17 - 10);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 10) & 131071;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 27 | (input[5 + inpos as usize] & 511) << (17 - 9);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 9) & 131071;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 26 | (input[6 + inpos as usize] & 255) << (17 - 8);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 8) & 131071;
//     output[14 + outpos as usize] =
//         input[6 + inpos as usize] >> 25 | (input[7 + inpos as usize] & 127) << (17 - 7);
//     output[15 + outpos as usize] = (input[7 + inpos as usize] >> 7) & 131071;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 24 | (input[8 + inpos as usize] & 63) << (17 - 6);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 6) & 131071;
//     output[18 + outpos as usize] =
//         input[8 + inpos as usize] >> 23 | (input[9 + inpos as usize] & 31) << (17 - 5);
//     output[19 + outpos as usize] = (input[9 + inpos as usize] >> 5) & 131071;
//     output[20 + outpos as usize] =
//         input[9 + inpos as usize] >> 22 | (input[10 + inpos as usize] & 15) << (17 - 4);
//     output[21 + outpos as usize] = (input[10 + inpos as usize] >> 4) & 131071;
//     output[22 + outpos as usize] =
//         input[10 + inpos as usize] >> 21 | (input[11 + inpos as usize] & 7) << (17 - 3);
//     output[23 + outpos as usize] = (input[11 + inpos as usize] >> 3) & 131071;
//     output[24 + outpos as usize] =
//         input[11 + inpos as usize] >> 20 | (input[12 + inpos as usize] & 3) << (17 - 2);
//     output[25 + outpos as usize] = (input[12 + inpos as usize] >> 2) & 131071;
//     output[26 + outpos as usize] = (input[12 + inpos as usize] >> 19) & 131071;
//     output[27 + outpos as usize] =
//         input[12 + inpos as usize] >> 36 | (input[13 + inpos as usize] & 32767) << (17 - 15);
//     output[28 + outpos as usize] = (input[13 + inpos as usize] >> 15) & 131071;
//     output[29 + outpos as usize] =
//         input[13 + inpos as usize] >> 32 | (input[14 + inpos as usize] & 16383) << (17 - 14);
//     output[30 + outpos as usize] = (input[14 + inpos as usize] >> 14) & 131071;
//     output[31 + outpos as usize] =
//         input[14 + inpos as usize] >> 31 | (input[15 + inpos as usize] & 8191) << (17 - 13);
// }
//
// fn fast_unpack18(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 262143;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 18) & 262143;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 36 | (input[1 + inpos as usize] & 16383) << (18 - 14);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 14) & 262143;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 32 | (input[2 + inpos as usize] & 8191) << (18 - 13);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 13) & 262143;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 31 | (input[3 + inpos as usize] & 4095) << (18 - 12);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 12) & 262143;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 30 | (input[4 + inpos as usize] & 2047) << (18 - 11);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 11) & 262143;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 29 | (input[5 + inpos as usize] & 1023) << (18 - 10);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 10) & 262143;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 28 | (input[6 + inpos as usize] & 511) << (18 - 9);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 9) & 262143;
//     output[14 + outpos as usize] =
//         input[6 + inpos as usize] >> 27 | (input[7 + inpos as usize] & 255) << (18 - 8);
//     output[15 + outpos as usize] = (input[7 + inpos as usize] >> 8) & 262143;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 26 | (input[8 + inpos as usize] & 127) << (18 - 7);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 7) & 262143;
//     output[18 + outpos as usize] =
//         input[8 + inpos as usize] >> 25 | (input[9 + inpos as usize] & 63) << (18 - 6);
//     output[19 + outpos as usize] = (input[9 + inpos as usize] >> 6) & 262143;
//     output[20 + outpos as usize] =
//         input[9 + inpos as usize] >> 24 | (input[10 + inpos as usize] & 31) << (18 - 5);
//     output[21 + outpos as usize] = (input[10 + inpos as usize] >> 5) & 262143;
//     output[22 + outpos as usize] =
//         input[10 + inpos as usize] >> 23 | (input[11 + inpos as usize] & 15) << (18 - 4);
//     output[23 + outpos as usize] = (input[11 + inpos as usize] >> 4) & 262143;
//     output[24 + outpos as usize] =
//         input[11 + inpos as usize] >> 22 | (input[12 + inpos as usize] & 7) << (18 - 3);
//     output[25 + outpos as usize] = (input[12 + inpos as usize] >> 3) & 262143;
//     output[26 + outpos as usize] =
//         input[12 + inpos as usize] >> 21 | (input[13 + inpos as usize] & 3) << (18 - 2);
//     output[27 + outpos as usize] = (input[13 + inpos as usize] >> 2) & 262143;
//     output[28 + outpos as usize] = (input[13 + inpos as usize] >> 20) & 262143;
//     output[29 + outpos as usize] =
//         input[13 + inpos as usize] >> 38 | (input[14 + inpos as usize] & 65535) << (18 - 16);
//     output[30 + outpos as usize] = (input[14 + inpos as usize] >> 16) & 262143;
//     output[31 + outpos as usize] =
//         input[14 + inpos as usize] >> 34 | (input[15 + inpos as usize] & 32767) << (18 - 15);
// }
//
// fn fast_unpack19(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 524287;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 19) & 524287;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 38 | (input[1 + inpos as usize] & 262143) << (19 - 18);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 18) & 524287;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 37 | (input[2 + inpos as usize] & 131071) << (19 - 17);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 17) & 524287;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 36 | (input[3 + inpos as usize] & 65535) << (19 - 16);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 16) & 524287;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 35 | (input[4 + inpos as usize] & 32767) << (19 - 15);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 15) & 524287;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 34 | (input[5 + inpos as usize] & 16383) << (19 - 14);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 14) & 524287;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 33 | (input[6 + inpos as usize] & 8191) << (19 - 13);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 13) & 524287;
//     output[14 + outpos as usize] =
//         input[6 + inpos as usize] >> 32 | (input[7 + inpos as usize] & 4095) << (19 - 12);
//     output[15 + outpos as usize] = (input[7 + inpos as usize] >> 12) & 524287;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 31 | (input[8 + inpos as usize] & 2047) << (19 - 11);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 11) & 524287;
//     output[18 + outpos as usize] =
//         input[8 + inpos as usize] >> 30 | (input[9 + inpos as usize] & 1023) << (19 - 10);
//     output[19 + outpos as usize] = (input[9 + inpos as usize] >> 10) & 524287;
//     output[20 + outpos as usize] =
//         input[9 + inpos as usize] >> 29 | (input[10 + inpos as usize] & 511) << (19 - 9);
//     output[21 + outpos as usize] = (input[10 + inpos as usize] >> 9) & 524287;
//     output[22 + outpos as usize] =
//         input[10 + inpos as usize] >> 28 | (input[11 + inpos as usize] & 255) << (19 - 8);
//     output[23 + outpos as usize] = (input[11 + inpos as usize] >> 8) & 524287;
//     output[24 + outpos as usize] =
//         input[11 + inpos as usize] >> 27 | (input[12 + inpos as usize] & 127) << (19 - 7);
//     output[25 + outpos as usize] = (input[12 + inpos as usize] >> 7) & 524287;
//     output[26 + outpos as usize] =
//         input[12 + inpos as usize] >> 26 | (input[13 + inpos as usize] & 63) << (19 - 6);
//     output[27 + outpos as usize] = (input[13 + inpos as usize] >> 6) & 524287;
//     output[28 + outpos as usize] =
//         input[13 + inpos as usize] >> 25 | (input[14 + inpos as usize] & 31) << (19 - 5);
//     output[29 + outpos as usize] = (input[14 + inpos as usize] >> 5) & 524287;
//     output[30 + outpos as usize] =
//         input[14 + inpos as usize] >> 24 | (input[15 + inpos as usize] & 15) << (19 - 4);
//     output[31 + outpos as usize] = (input[15 + inpos as usize] >> 4) & 524287;
// }
//
// fn fast_unpack20(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 1048575;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 20) & 1048575;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 40 | (input[1 + inpos as usize] & 65535) << (20 - 16);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 16) & 1048575;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 36 | (input[2 + inpos as usize] & 4095) << (20 - 12);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 12) & 1048575;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 32 | (input[3 + inpos as usize] & 255) << (20 - 8);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 8) & 1048575;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 28 | (input[4 + inpos as usize] & 15) << (20 - 4);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 4) & 1048575;
//     output[10 + outpos as usize] = (input[4 + inpos as usize] >> 24) & 1048575;
//     output[11 + outpos as usize] =
//         input[4 + inpos as usize] >> 44 | (input[5 + inpos as usize] & 262143) << (20 - 18);
//     output[12 + outpos as usize] = (input[5 + inpos as usize] >> 18) & 1048575;
//     output[13 + outpos as usize] =
//         input[5 + inpos as usize] >> 38 | (input[6 + inpos as usize] & 16383) << (20 - 14);
//     output[14 + outpos as usize] = (input[6 + inpos as usize] >> 14) & 1048575;
//     output[15 + outpos as usize] =
//         input[6 + inpos as usize] >> 34 | (input[7 + inpos as usize] & 1023) << (20 - 10);
//     output[16 + outpos as usize] = (input[7 + inpos as usize] >> 10) & 1048575;
//     output[17 + outpos as usize] =
//         input[7 + inpos as usize] >> 30 | (input[8 + inpos as usize] & 63) << (20 - 6);
//     output[18 + outpos as usize] = (input[8 + inpos as usize] >> 6) & 1048575;
//     output[19 + outpos as usize] =
//         input[8 + inpos as usize] >> 26 | (input[9 + inpos as usize] & 3) << (20 - 2);
//     output[20 + outpos as usize] = (input[9 + inpos as usize] >> 2) & 1048575;
//     output[21 + outpos as usize] = (input[9 + inpos as usize] >> 22) & 1048575;
//     output[22 + outpos as usize] =
//         input[9 + inpos as usize] >> 42 | (input[10 + inpos as usize] & 131071) << (20 - 17);
//     output[23 + outpos as usize] = (input[10 + inpos as usize] >> 17) & 1048575;
//     output[24 + outpos as usize] =
//         input[10 + inpos as usize] >> 37 | (input[11 + inpos as usize] & 8191) << (20 - 13);
//     output[25 + outpos as usize] = (input[11 + inpos as usize] >> 13) & 1048575;
//     output[26 + outpos as usize] =
//         input[11 + inpos as usize] >> 33 | (input[12 + inpos as usize] & 511) << (20 - 9);
//     output[27 + outpos as usize] = (input[12 + inpos as usize] >> 9) & 1048575;
//     output[28 + outpos as usize] =
//         input[12 + inpos as usize] >> 29 | (input[13 + inpos as usize] & 31) << (20 - 5);
//     output[29 + outpos as usize] = (input[13 + inpos as usize] >> 5) & 1048575;
//     output[30 + outpos as usize] =
//         input[13 + inpos as usize] >> 25 | (input[14 + inpos as usize] & 1) << (20 - 1);
//     output[31 + outpos as usize] = (input[14 + inpos as usize] >> 1) & 1048575;
// }
//
// fn fast_unpack21(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 2097151;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 21) & 2097151;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 42 | (input[1 + inpos as usize] & 1048575) << (21 - 20);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 20) & 2097151;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 41 | (input[2 + inpos as usize] & 524287) << (21 - 19);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 19) & 2097151;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 40 | (input[3 + inpos as usize] & 262143) << (21 - 18);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 18) & 2097151;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 39 | (input[4 + inpos as usize] & 131071) << (21 - 17);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 17) & 2097151;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 38 | (input[5 + inpos as usize] & 65535) << (21 - 16);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 16) & 2097151;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 37 | (input[6 + inpos as usize] & 32767) << (21 - 15);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 15) & 2097151;
//     output[14 + outpos as usize] =
//         input[6 + inpos as usize] >> 36 | (input[7 + inpos as usize] & 16383) << (21 - 14);
//     output[15 + outpos as usize] = (input[7 + inpos as usize] >> 14) & 2097151;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 35 | (input[8 + inpos as usize] & 8191) << (21 - 13);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 13) & 2097151;
//     output[18 + outpos as usize] =
//         input[8 + inpos as usize] >> 34 | (input[9 + inpos as usize] & 4095) << (21 - 12);
//     output[19 + outpos as usize] = (input[9 + inpos as usize] >> 12) & 2097151;
//     output[20 + outpos as usize] =
//         input[9 + inpos as usize] >> 33 | (input[10 + inpos as usize] & 2047) << (21 - 11);
//     output[21 + outpos as usize] = (input[10 + inpos as usize] >> 11) & 2097151;
//     output[22 + outpos as usize] =
//         input[10 + inpos as usize] >> 32 | (input[11 + inpos as usize] & 1023) << (21 - 10);
//     output[23 + outpos as usize] = (input[11 + inpos as usize] >> 10) & 2097151;
//     output[24 + outpos as usize] =
//         input[11 + inpos as usize] >> 31 | (input[12 + inpos as usize] & 511) << (21 - 9);
//     output[25 + outpos as usize] = (input[12 + inpos as usize] >> 9) & 2097151;
//     output[26 + outpos as usize] =
//         input[12 + inpos as usize] >> 30 | (input[13 + inpos as usize] & 255) << (21 - 8);
//     output[27 + outpos as usize] = (input[13 + inpos as usize] >> 8) & 2097151;
//     output[28 + outpos as usize] =
//         input[13 + inpos as usize] >> 29 | (input[14 + inpos as usize] & 127) << (21 - 7);
//     output[29 + outpos as usize] = (input[14 + inpos as usize] >> 7) & 2097151;
//     output[30 + outpos as usize] =
//         input[14 + inpos as usize] >> 28 | (input[15 + inpos as usize] & 63) << (21 - 6);
//     output[31 + outpos as usize] = (input[15 + inpos as usize] >> 6) & 2097151;
// }
//
// fn fast_unpack22(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 4194303;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 22) & 4194303;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 44 | (input[1 + inpos as usize] & 1048575) << (22 - 20);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 20) & 4194303;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 42 | (input[2 + inpos as usize] & 262143) << (22 - 18);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 18) & 4194303;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 40 | (input[3 + inpos as usize] & 65535) << (22 - 16);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 16) & 4194303;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 38 | (input[4 + inpos as usize] & 16383) << (22 - 14);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 14) & 4194303;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 36 | (input[5 + inpos as usize] & 4095) << (22 - 12);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 12) & 4194303;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 34 | (input[6 + inpos as usize] & 1023) << (22 - 10);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 10) & 4194303;
//     output[14 + outpos as usize] =
//         input[6 + inpos as usize] >> 32 | (input[7 + inpos as usize] & 255) << (22 - 8);
//     output[15 + outpos as usize] = (input[7 + inpos as usize] >> 8) & 4194303;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 30 | (input[8 + inpos as usize] & 63) << (22 - 6);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 6) & 4194303;
//     output[18 + outpos as usize] =
//         input[8 + inpos as usize] >> 28 | (input[9 + inpos as usize] & 15) << (22 - 4);
//     output[19 + outpos as usize] = (input[9 + inpos as usize] >> 4) & 4194303;
//     output[20 + outpos as usize] = (input[9 + inpos as usize] >> 26) & 4194303;
//     output[21 + outpos as usize] =
//         input[9 + inpos as usize] >> 48 | (input[10 + inpos as usize] & 262143) << (22 - 18);
//     output[22 + outpos as usize] = (input[10 + inpos as usize] >> 18) & 4194303;
//     output[23 + outpos as usize] =
//         input[10 + inpos as usize] >> 40 | (input[11 + inpos as usize] & 65535) << (22 - 16);
//     output[24 + outpos as usize] = (input[11 + inpos as usize] >> 16) & 4194303;
//     output[25 + outpos as usize] =
//         input[11 + inpos as usize] >> 38 | (input[12 + inpos as usize] & 16383) << (22 - 14);
//     output[26 + outpos as usize] = (input[12 + inpos as usize] >> 14) & 4194303;
//     output[27 + outpos as usize] =
//         input[12 + inpos as usize] >> 36 | (input[13 + inpos as usize] & 4095) << (22 - 12);
//     output[28 + outpos as usize] = (input[13 + inpos as usize] >> 12) & 4194303;
//     output[29 + outpos as usize] =
//         input[13 + inpos as usize] >> 34 | (input[14 + inpos as usize] & 1023) << (22 - 10);
//     output[30 + outpos as usize] = (input[14 + inpos as usize] >> 10) & 4194303;
//     output[31 + outpos as usize] =
//         input[14 + inpos as usize] >> 32 | (input[15 + inpos as usize] & 255) << (22 - 8);
// }
//
// fn fast_unpack23(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 8388607;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 23) & 8388607;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 46 | (input[1 + inpos as usize] & 4194303) << (23 - 22);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 22) & 8388607;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 45 | (input[2 + inpos as usize] & 2097151) << (23 - 21);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 21) & 8388607;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 44 | (input[3 + inpos as usize] & 1048575) << (23 - 20);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 20) & 8388607;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 43 | (input[4 + inpos as usize] & 524287) << (23 - 19);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 19) & 8388607;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 42 | (input[5 + inpos as usize] & 262143) << (23 - 18);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 18) & 8388607;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 41 | (input[6 + inpos as usize] & 131071) << (23 - 17);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 17) & 8388607;
//     output[14 + outpos as usize] =
//         input[6 + inpos as usize] >> 40 | (input[7 + inpos as usize] & 65535) << (23 - 16);
//     output[15 + outpos as usize] = (input[7 + inpos as usize] >> 16) & 8388607;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 39 | (input[8 + inpos as usize] & 32767) << (23 - 15);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 15) & 8388607;
//     output[18 + outpos as usize] =
//         input[8 + inpos as usize] >> 38 | (input[9 + inpos as usize] & 16383) << (23 - 14);
//     output[19 + outpos as usize] = (input[9 + inpos as usize] >> 14) & 8388607;
//     output[20 + outpos as usize] =
//         input[9 + inpos as usize] >> 37 | (input[10 + inpos as usize] & 8191) << (23 - 13);
//     output[21 + outpos as usize] = (input[10 + inpos as usize] >> 13) & 8388607;
//     output[22 + outpos as usize] =
//         input[10 + inpos as usize] >> 36 | (input[11 + inpos as usize] & 4095) << (23 - 12);
//     output[23 + outpos as usize] = (input[11 + inpos as usize] >> 12) & 8388607;
//     output[24 + outpos as usize] =
//         input[11 + inpos as usize] >> 35 | (input[12 + inpos as usize] & 2047) << (23 - 11);
//     output[25 + outpos as usize] = (input[12 + inpos as usize] >> 11) & 8388607;
//     output[26 + outpos as usize] =
//         input[12 + inpos as usize] >> 34 | (input[13 + inpos as usize] & 1023) << (23 - 10);
//     output[27 + outpos as usize] = (input[13 + inpos as usize] >> 10) & 8388607;
//     output[28 + outpos as usize] =
//         input[13 + inpos as usize] >> 33 | (input[14 + inpos as usize] & 511) << (23 - 9);
//     output[29 + outpos as usize] = (input[14 + inpos as usize] >> 9) & 8388607;
//     output[30 + outpos as usize] =
//         input[14 + inpos as usize] >> 32 | (input[15 + inpos as usize] & 255) << (23 - 8);
//     output[31 + outpos as usize] = (input[15 + inpos as usize] >> 8) & 8388607;
// }
//
// fn fast_unpack24(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 16777215;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 24) & 16777215;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 48 | (input[1 + inpos as usize] & 65535) << (24 - 16);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 16) & 16777215;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 40 | (input[2 + inpos as usize] & 4095) << (24 - 12);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 12) & 16777215;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 36 | (input[3 + inpos as usize] & 255) << (24 - 8);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 8) & 16777215;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 32 | (input[4 + inpos as usize] & 15) << (24 - 4);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 4) & 16777215;
//     output[10 + outpos as usize] = (input[4 + inpos as usize] >> 28) & 16777215;
//     output[11 + outpos as usize] =
//         input[4 + inpos as usize] >> 52 | (input[5 + inpos as usize] & 1048575) << (24 - 20);
//     output[12 + outpos as usize] = (input[5 + inpos as usize] >> 20) & 16777215;
//     output[13 + outpos as usize] =
//         input[5 + inpos as usize] >> 44 | (input[6 + inpos as usize] & 4095) << (24 - 12);
//     output[14 + outpos as usize] = (input[6 + inpos as usize] >> 12) & 16777215;
//     output[15 + outpos as usize] =
//         input[6 + inpos as usize] >> 36 | (input[7 + inpos as usize] & 255) << (24 - 8);
//     output[16 + outpos as usize] = (input[7 + inpos as usize] >> 8) & 16777215;
//     output[17 + outpos as usize] =
//         input[7 + inpos as usize] >> 32 | (input[8 + inpos as usize] & 15) << (24 - 4);
//     output[18 + outpos as usize] = (input[8 + inpos as usize] >> 4) & 16777215;
//     output[19 + outpos as usize] = (input[8 + inpos as usize] >> 28) & 16777215;
//     output[20 + outpos as usize] =
//         input[8 + inpos as usize] >> 52 | (input[9 + inpos as usize] & 1048575) << (24 - 20);
//     output[21 + outpos as usize] = (input[9 + inpos as usize] >> 20) & 16777215;
//     output[22 + outpos as usize] =
//         input[9 + inpos as usize] >> 44 | (input[10 + inpos as usize] & 4095) << (24 - 12);
//     output[23 + outpos as usize] = (input[10 + inpos as usize] >> 12) & 16777215;
//     output[24 + outpos as usize] =
//         input[10 + inpos as usize] >> 36 | (input[11 + inpos as usize] & 255) << (24 - 8);
//     output[25 + outpos as usize] = (input[11 + inpos as usize] >> 8) & 16777215;
//     output[26 + outpos as usize] =
//         input[11 + inpos as usize] >> 32 | (input[12 + inpos as usize] & 15) << (24 - 4);
//     output[27 + outpos as usize] = (input[12 + inpos as usize] >> 4) & 16777215;
//     output[28 + outpos as usize] = (input[12 + inpos as usize] >> 28) & 16777215;
//     output[29 + outpos as usize] =
//         input[12 + inpos as usize] >> 52 | (input[13 + inpos as usize] & 1048575) << (24 - 20);
//     output[30 + outpos as usize] = (input[13 + inpos as usize] >> 20) & 16777215;
//     output[31 + outpos as usize] =
//         input[13 + inpos as usize] >> 44 | (input[14 + inpos as usize] & 4095) << (24 - 12);
// }
//
// fn fast_unpack25(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 33554431;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 25) & 33554431;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 50 | (input[1 + inpos as usize] & 131071) << (25 - 17);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 17) & 33554431;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 42 | (input[2 + inpos as usize] & 511) << (25 - 9);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 9) & 33554431;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 34 | (input[3 + inpos as usize] & 31) << (25 - 5);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 5) & 33554431;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 30 | (input[4 + inpos as usize] & 1) << (25 - 1);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 1) & 33554431;
//     output[10 + outpos as usize] = (input[4 + inpos as usize] >> 26) & 33554431;
//     output[11 + outpos as usize] =
//         input[4 + inpos as usize] >> 51 | (input[5 + inpos as usize] & 262143) << (25 - 18);
//     output[12 + outpos as usize] = (input[5 + inpos as usize] >> 18) & 33554431;
//     output[13 + outpos as usize] =
//         input[5 + inpos as usize] >> 43 | (input[6 + inpos as usize] & 1023) << (25 - 10);
//     output[14 + outpos as usize] = (input[6 + inpos as usize] >> 10) & 33554431;
//     output[15 + outpos as usize] =
//         input[6 + inpos as usize] >> 35 | (input[7 + inpos as usize] & 63) << (25 - 6);
//     output[16 + outpos as usize] = (input[7 + inpos as usize] >> 6) & 33554431;
//     output[17 + outpos as usize] =
//         input[7 + inpos as usize] >> 31 | (input[8 + inpos as usize] & 3) << (25 - 2);
//     output[18 + outpos as usize] = (input[8 + inpos as usize] >> 2) & 33554431;
//     output[19 + outpos as usize] = (input[8 + inpos as usize] >> 27) & 33554431;
//     output[20 + outpos as usize] =
//         input[8 + inpos as usize] >> 52 | (input[9 + inpos as usize] & 524287) << (25 - 19);
//     output[21 + outpos as usize] = (input[9 + inpos as usize] >> 19) & 33554431;
//     output[22 + outpos as usize] =
//         input[9 + inpos as usize] >> 44 | (input[10 + inpos as usize] & 2047) << (25 - 11);
//     output[23 + outpos as usize] = (input[10 + inpos as usize] >> 11) & 33554431;
//     output[24 + outpos as usize] =
//         input[10 + inpos as usize] >> 36 | (input[11 + inpos as usize] & 127) << (25 - 7);
//     output[25 + outpos as usize] = (input[11 + inpos as usize] >> 7) & 33554431;
//     output[26 + outpos as usize] =
//         input[11 + inpos as usize] >> 32 | (input[12 + inpos as usize] & 7) << (25 - 3);
//     output[27 + outpos as usize] = (input[12 + inpos as usize] >> 3) & 33554431;
//     output[28 + outpos as usize] = (input[12 + inpos as usize] >> 28) & 33554431;
//     output[29 + outpos as usize] =
//         input[12 + inpos as usize] >> 53 | (input[13 + inpos as usize] & 1048575) << (25 - 20);
//     output[30 + outpos as usize] = (input[13 + inpos as usize] >> 20) & 33554431;
//     output[31 + outpos as usize] =
//         input[13 + inpos as usize] >> 45 | (input[14 + inpos as usize] & 4095) << (25 - 12);
// }
//
// fn fast_unpack26(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 67108863;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 26) & 67108863;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 52 | (input[1 + inpos as usize] & 4194303) << (26 - 22);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 22) & 67108863;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 48 | (input[2 + inpos as usize] & 262143) << (26 - 18);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 18) & 67108863;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 44 | (input[3 + inpos as usize] & 16383) << (26 - 14);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 14) & 67108863;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 40 | (input[4 + inpos as usize] & 1023) << (26 - 10);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 10) & 67108863;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 36 | (input[5 + inpos as usize] & 63) << (26 - 6);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 6) & 67108863;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 32 | (input[6 + inpos as usize] & 15) << (26 - 4);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 4) & 67108863;
//     output[14 + outpos as usize] = (input[6 + inpos as usize] >> 30) & 67108863;
//     output[15 + outpos as usize] =
//         input[6 + inpos as usize] >> 56 | (input[7 + inpos as usize] & 16777215) << (26 - 24);
//     output[16 + outpos as usize] = (input[7 + inpos as usize] >> 24) & 67108863;
//     output[17 + outpos as usize] =
//         input[7 + inpos as usize] >> 50 | (input[8 + inpos as usize] & 1048575) << (26 - 20);
//     output[18 + outpos as usize] = (input[8 + inpos as usize] >> 20) & 67108863;
//     output[19 + outpos as usize] =
//         input[8 + inpos as usize] >> 46 | (input[9 + inpos as usize] & 65535) << (26 - 16);
//     output[20 + outpos as usize] = (input[9 + inpos as usize] >> 16) & 67108863;
//     output[21 + outpos as usize] =
//         input[9 + inpos as usize] >> 42 | (input[10 + inpos as usize] & 4095) << (26 - 12);
//     output[22 + outpos as usize] = (input[10 + inpos as usize] >> 12) & 67108863;
//     output[23 + outpos as usize] =
//         input[10 + inpos as usize] >> 38 | (input[11 + inpos as usize] & 255) << (26 - 8);
//     output[24 + outpos as usize] = (input[11 + inpos as usize] >> 8) & 67108863;
//     output[25 + outpos as usize] =
//         input[11 + inpos as usize] >> 34 | (input[12 + inpos as usize] & 15) << (26 - 4);
//     output[26 + outpos as usize] = (input[12 + inpos as usize] >> 4) & 67108863;
//     output[27 + outpos as usize] = (input[12 + inpos as usize] >> 30) & 67108863;
//     output[28 + outpos as usize] =
//         input[12 + inpos as usize] >> 56 | (input[13 + inpos as usize] & 16777215) << (26 - 24);
//     output[29 + outpos as usize] = (input[13 + inpos as usize] >> 24) & 67108863;
//     output[30 + outpos as usize] =
//         input[13 + inpos as usize] >> 50 | (input[14 + inpos as usize] & 1048575) << (26 - 20);
//     output[31 + outpos as usize] = (input[14 + inpos as usize] >> 20) & 67108863;
// }
//
// fn fast_unpack27(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 134217727;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 27) & 134217727;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 54 | (input[1 + inpos as usize] & 33554431) << (27 - 25);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 25) & 134217727;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 52 | (input[2 + inpos as usize] & 8388607) << (27 - 23);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 23) & 134217727;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 50 | (input[3 + inpos as usize] & 2097151) << (27 - 21);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 21) & 134217727;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 48 | (input[4 + inpos as usize] & 524287) << (27 - 19);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 19) & 134217727;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 46 | (input[5 + inpos as usize] & 131071) << (27 - 17);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 17) & 134217727;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 44 | (input[6 + inpos as usize] & 32767) << (27 - 15);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 15) & 134217727;
//     output[14 + outpos as usize] =
//         input[6 + inpos as usize] >> 42 | (input[7 + inpos as usize] & 8191) << (27 - 13);
//     output[15 + outpos as usize] = (input[7 + inpos as usize] >> 13) & 134217727;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 40 | (input[8 + inpos as usize] & 2047) << (27 - 11);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 11) & 134217727;
//     output[18 + outpos as usize] =
//         input[8 + inpos as usize] >> 38 | (input[9 + inpos as usize] & 511) << (27 - 9);
//     output[19 + outpos as usize] = (input[9 + inpos as usize] >> 9) & 134217727;
//     output[20 + outpos as usize] =
//         input[9 + inpos as usize] >> 36 | (input[10 + inpos as usize] & 127) << (27 - 7);
//     output[21 + outpos as usize] = (input[10 + inpos as usize] >> 7) & 134217727;
//     output[22 + outpos as usize] =
//         input[10 + inpos as usize] >> 34 | (input[11 + inpos as usize] & 63) << (27 - 6);
//     output[23 + outpos as usize] = (input[11 + inpos as usize] >> 6) & 134217727;
//     output[24 + outpos as usize] =
//         input[11 + inpos as usize] >> 33 | (input[12 + inpos as usize] & 31) << (27 - 5);
//     output[25 + outpos as usize] = (input[12 + inpos as usize] >> 5) & 134217727;
//     output[26 + outpos as usize] =
//         input[12 + inpos as usize] >> 32 | (input[13 + inpos as usize] & 15) << (27 - 4);
//     output[27 + outpos as usize] = (input[13 + inpos as usize] >> 4) & 134217727;
//     output[28 + outpos as usize] =
//         input[13 + inpos as usize] >> 31 | (input[14 + inpos as usize] & 7) << (27 - 3);
//     output[29 + outpos as usize] = (input[14 + inpos as usize] >> 3) & 134217727;
//     output[30 + outpos as usize] =
//         input[14 + inpos as usize] >> 30 | (input[15 + inpos as usize] & 3) << (27 - 2);
//     output[31 + outpos as usize] = (input[15 + inpos as usize] >> 2) & 134217727;
// }
//
// fn fast_unpack28(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 268435455;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 28) & 268435455;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 56 | (input[1 + inpos as usize] & 16777215) << (28 - 24);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 24) & 268435455;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 52 | (input[2 + inpos as usize] & 1048575) << (28 - 20);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 20) & 268435455;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 48 | (input[3 + inpos as usize] & 65535) << (28 - 16);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 16) & 268435455;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 44 | (input[4 + inpos as usize] & 4095) << (28 - 12);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 12) & 268435455;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 40 | (input[5 + inpos as usize] & 255) << (28 - 8);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 8) & 268435455;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 36 | (input[6 + inpos as usize] & 15) << (28 - 4);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 4) & 268435455;
//     output[14 + outpos as usize] = (input[6 + inpos as usize] >> 32) & 268435455;
//     output[15 + outpos as usize] =
//         input[6 + inpos as usize] >> 60 | (input[7 + inpos as usize] & 67108863) << (28 - 26);
//     output[16 + outpos as usize] = (input[7 + inpos as usize] >> 26) & 268435455;
//     output[17 + outpos as usize] =
//         input[7 + inpos as usize] >> 54 | (input[8 + inpos as usize] & 4194303) << (28 - 22);
//     output[18 + outpos as usize] = (input[8 + inpos as usize] >> 22) & 268435455;
//     output[19 + outpos as usize] =
//         input[8 + inpos as usize] >> 50 | (input[9 + inpos as usize] & 262143) << (28 - 18);
//     output[20 + outpos as usize] = (input[9 + inpos as usize] >> 18) & 268435455;
//     output[21 + outpos as usize] =
//         input[9 + inpos as usize] >> 46 | (input[10 + inpos as usize] & 16383) << (28 - 14);
//     output[22 + outpos as usize] = (input[10 + inpos as usize] >> 14) & 268435455;
//     output[23 + outpos as usize] =
//         input[10 + inpos as usize] >> 42 | (input[11 + inpos as usize] & 1023) << (28 - 10);
//     output[24 + outpos as usize] = (input[11 + inpos as usize] >> 10) & 268435455;
//     output[25 + outpos as usize] =
//         input[11 + inpos as usize] >> 38 | (input[12 + inpos as usize] & 63) << (28 - 6);
//     output[26 + outpos as usize] = (input[12 + inpos as usize] >> 6) & 268435455;
//     output[27 + outpos as usize] =
//         input[12 + inpos as usize] >> 34 | (input[13 + inpos as usize] & 31) << (28 - 5);
//     output[28 + outpos as usize] = (input[13 + inpos as usize] >> 5) & 268435455;
//     output[29 + outpos as usize] =
//         input[13 + inpos as usize] >> 62 | (input[14 + inpos as usize] & 268435455) << (28 - 30);
//     output[30 + outpos as usize] = (input[14 + inpos as usize] >> 30) & 268435455;
//     output[31 + outpos as usize] =
//         input[14 + inpos as usize] >> 58 | (input[15 + inpos as usize] & 16777215) << (28 - 24);
// }
//
// fn fast_unpack29(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 536870911;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 29) & 536870911;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 58 | (input[1 + inpos as usize] & 67108863) << (29 - 26);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 26) & 536870911;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 55 | (input[2 + inpos as usize] & 33554431) << (29 - 25);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 25) & 536870911;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 54 | (input[3 + inpos as usize] & 16777215) << (29 - 24);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 24) & 536870911;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 53 | (input[4 + inpos as usize] & 8388607) << (29 - 23);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 23) & 536870911;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 52 | (input[5 + inpos as usize] & 4194303) << (29 - 22);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 22) & 536870911;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 51 | (input[6 + inpos as usize] & 2097151) << (29 - 21);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 21) & 536870911;
//     output[14 + outpos as usize] =
//         input[6 + inpos as usize] >> 50 | (input[7 + inpos as usize] & 1048575) << (29 - 20);
//     output[15 + outpos as usize] = (input[7 + inpos as usize] >> 20) & 536870911;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 49 | (input[8 + inpos as usize] & 524287) << (29 - 19);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 19) & 536870911;
//     output[18 + outpos as usize] =
//         input[8 + inpos as usize] >> 48 | (input[9 + inpos as usize] & 262143) << (29 - 18);
//     output[19 + outpos as usize] = (input[9 + inpos as usize] >> 18) & 536870911;
//     output[20 + outpos as usize] =
//         input[9 + inpos as usize] >> 47 | (input[10 + inpos as usize] & 131071) << (29 - 17);
//     output[21 + outpos as usize] = (input[10 + inpos as usize] >> 17) & 536870911;
//     output[22 + outpos as usize] =
//         input[10 + inpos as usize] >> 46 | (input[11 + inpos as usize] & 65535) << (29 - 16);
//     output[23 + outpos as usize] = (input[11 + inpos as usize] >> 16) & 536870911;
//     output[24 + outpos as usize] =
//         input[11 + inpos as usize] >> 45 | (input[12 + inpos as usize] & 32767) << (29 - 15);
//     output[25 + outpos as usize] = (input[12 + inpos as usize] >> 15) & 536870911;
//     output[26 + outpos as usize] =
//         input[12 + inpos as usize] >> 44 | (input[13 + inpos as usize] & 16383) << (29 - 14);
//     output[27 + outpos as usize] = (input[13 + inpos as usize] >> 14) & 536870911;
//     output[28 + outpos as usize] =
//         input[13 + inpos as usize] >> 43 | (input[14 + inpos as usize] & 8191) << (29 - 13);
//     output[29 + outpos as usize] = (input[14 + inpos as usize] >> 13) & 536870911;
//     output[30 + outpos as usize] =
//         input[14 + inpos as usize] >> 42 | (input[15 + inpos as usize] & 4095) << (29 - 12);
//     output[31 + outpos as usize] = (input[15 + inpos as usize] >> 12) & 536870911;
// }
//
// fn fast_unpack30(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 1073741823;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 30) & 1073741823;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 60 | (input[1 + inpos as usize] & 268435455) << (30 - 28);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 28) & 1073741823;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 58 | (input[2 + inpos as usize] & 67108863) << (30 - 26);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 26) & 1073741823;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 56 | (input[3 + inpos as usize] & 16777215) << (30 - 24);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 24) & 1073741823;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 54 | (input[4 + inpos as usize] & 4194303) << (30 - 22);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 22) & 1073741823;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 52 | (input[5 + inpos as usize] & 1048575) << (30 - 20);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 20) & 1073741823;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 50 | (input[6 + inpos as usize] & 262143) << (30 - 18);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 18) & 1073741823;
//     output[14 + outpos as usize] =
//         input[6 + inpos as usize] >> 48 | (input[7 + inpos as usize] & 65535) << (30 - 16);
//     output[15 + outpos as usize] = (input[7 + inpos as usize] >> 16) & 1073741823;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 46 | (input[8 + inpos as usize] & 16383) << (30 - 14);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 14) & 1073741823;
//     output[18 + outpos as usize] =
//         input[8 + inpos as usize] >> 44 | (input[9 + inpos as usize] & 4095) << (30 - 12);
//     output[19 + outpos as usize] = (input[9 + inpos as usize] >> 12) & 1073741823;
//     output[20 + outpos as usize] =
//         input[9 + inpos as usize] >> 42 | (input[10 + inpos as usize] & 1023) << (30 - 10);
//     output[21 + outpos as usize] = (input[10 + inpos as usize] >> 10) & 1073741823;
//     output[22 + outpos as usize] =
//         input[10 + inpos as usize] >> 40 | (input[11 + inpos as usize] & 255) << (30 - 8);
//     output[23 + outpos as usize] = (input[11 + inpos as usize] >> 8) & 1073741823;
//     output[24 + outpos as usize] =
//         input[11 + inpos as usize] >> 38 | (input[12 + inpos as usize] & 63) << (30 - 6);
//     output[25 + outpos as usize] = (input[12 + inpos as usize] >> 6) & 1073741823;
//     output[26 + outpos as usize] =
//         input[12 + inpos as usize] >> 36 | (input[13 + inpos as usize] & 15) << (30 - 4);
//     output[27 + outpos as usize] = (input[13 + inpos as usize] >> 4) & 1073741823;
//     output[28 + outpos as usize] =
//         input[13 + inpos as usize] >> 34 | (input[14 + inpos as usize] & 3) << (30 - 2);
//     output[29 + outpos as usize] = (input[14 + inpos as usize] >> 2) & 1073741823;
//     output[30 + outpos as usize] =
//         input[14 + inpos as usize] >> 32 | (input[15 + inpos as usize] & 1073741823) << (30 - 30);
//     output[31 + outpos as usize] = (input[15 + inpos as usize] >> 30) & 1073741823;
// }
//
// fn fast_unpack31(input: &mut [i32], inpos: i32, output: &mut [i32], outpos: i32) {
//     output[0 + outpos as usize] = (input[0 + inpos as usize] >> 0) & 2147483647;
//     output[1 + outpos as usize] = (input[0 + inpos as usize] >> 31) & 2147483647;
//     output[2 + outpos as usize] =
//         input[0 + inpos as usize] >> 62 | (input[1 + inpos as usize] & 1073741823) << (31 - 30);
//     output[3 + outpos as usize] = (input[1 + inpos as usize] >> 30) & 2147483647;
//     output[4 + outpos as usize] =
//         input[1 + inpos as usize] >> 61 | (input[2 + inpos as usize] & 536870911) << (31 - 29);
//     output[5 + outpos as usize] = (input[2 + inpos as usize] >> 29) & 2147483647;
//     output[6 + outpos as usize] =
//         input[2 + inpos as usize] >> 60 | (input[3 + inpos as usize] & 268435455) << (31 - 28);
//     output[7 + outpos as usize] = (input[3 + inpos as usize] >> 28) & 2147483647;
//     output[8 + outpos as usize] =
//         input[3 + inpos as usize] >> 59 | (input[4 + inpos as usize] & 134217727) << (31 - 27);
//     output[9 + outpos as usize] = (input[4 + inpos as usize] >> 27) & 2147483647;
//     output[10 + outpos as usize] =
//         input[4 + inpos as usize] >> 58 | (input[5 + inpos as usize] & 67108863) << (31 - 26);
//     output[11 + outpos as usize] = (input[5 + inpos as usize] >> 26) & 2147483647;
//     output[12 + outpos as usize] =
//         input[5 + inpos as usize] >> 57 | (input[6 + inpos as usize] & 33554431) << (31 - 25);
//     output[13 + outpos as usize] = (input[6 + inpos as usize] >> 25) & 2147483647;
//     output[14 + outpos as usize] =
//         input[6 + inpos as usize] >> 56 | (input[7 + inpos as usize] & 16777215) << (31 - 24);
//     output[15 + outpos as usize] = (input[7 + inpos as usize] >> 24) & 2147483647;
//     output[16 + outpos as usize] =
//         input[7 + inpos as usize] >> 55 | (input[8 + inpos as usize] & 8388607) << (31 - 23);
//     output[17 + outpos as usize] = (input[8 + inpos as usize] >> 23) & 2147483647;
//     output[18 + outpos as usize] =
//         input[8 + inpos as usize] >> 54 | (input[9 + inpos as usize] & 4194303) << (31 - 22);
//     output[19 + outpos as usize] = (input[9 + inpos as usize] >> 22) & 2147483647;
//     output[20 + outpos as usize] =
//         input[9 + inpos as usize] >> 53 | (input[10 + inpos as usize] & 2097151) << (31 - 21);
//     output[21 + outpos as usize] = (input[10 + inpos as usize] >> 21) & 2147483647;
//     output[22 + outpos as usize] =
//         input[10 + inpos as usize] >> 52 | (input[11 + inpos as usize] & 1048575) << (31 - 20);
//     output[23 + outpos as usize] = (input[11 + inpos as usize] >> 20) & 2147483647;
//     output[24 + outpos as usize] =
//         input[11 + inpos as usize] >> 51 | (input[12 + inpos as usize] & 524287) << (31 - 19);
//     output[25 + outpos as usize] = (input[12 + inpos as usize] >> 19) & 2147483647;
//     output[26 + outpos as usize] =
//         input[12 + inpos as usize] >> 50 | (input[13 + inpos as usize] & 262143) << (31 - 18);
//     output[27 + outpos as usize] = (input[13 + inpos as usize] >> 18) & 2147483647;
//     output[28 + outpos as usize] =
//         input[13 + inpos as usize] >> 49 | (input[14 + inpos as usize] & 131071) << (31 - 17);
//     output[29 + outpos as usize] = (input[14 + inpos as usize] >> 17) & 2147483647;
//     output[30 + outpos as usize] =
//         input[14 + inpos as usize] >> 48 | (input[15 + inpos as usize] & 65535) << (31 - 16);
//     output[31 + outpos as usize] = (input[15 + inpos as usize] >> 16) & 2147483647;
// }
//
// fn fast_unpack32(input: &[i32], inpos: usize, output: &mut [i32], outpos: usize) {
//     output[outpos..outpos + 32].copy_from_slice(&input[inpos..inpos + 32]);
// }
