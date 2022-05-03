use bitvec::prelude::*;

fn main() {
    let bv: BitVec<Msb0, u8> = bitvec![Msb0, u8;  1, 1, 0, 0, 0, 0, 0, 0];
    let value = bv.load_be::<u128>() as i8;
    println!("bits_as_i8: {}", value);
    let value = bv.load_be::<u128>() as i16;
    println!("bits_as_i16: {}", value);

    let numbers: Vec<i128> = vec![
        140737488355328,
        140737488355327,
        549755813888,
        549755813887,
        2147483648,
        2147483647,
        8388608,
        8388607,
        32768,
        32767,
        128,
        127,
        1,
        0,
        -1,
        -128,
        -129,
        -32768,
        -32769,
        -8388608,
        -8388609,
        -2147483648,
        -2147483649,
        -549755813888,
        -549755813889,
        -140737488355328,
        -140737488355329,
    ];
    for number in numbers {
        let bytes_needed = if number < 0 {
            let leading_ones = number.leading_ones();

            if leading_ones % 8 == 0 {
                16 - leading_ones / 8 + 1
            } else {
                16 - leading_ones / 8
            }
        } else {
            let leading_zeroes = number.leading_zeros();
            if leading_zeroes % 8 == 0 {
                16 - leading_zeroes / 8 + 1
            } else {
                16 - leading_zeroes / 8
            }
        };

        eprintln!("number: {}, bytes_needed: {}", number, bytes_needed);
    }
}
