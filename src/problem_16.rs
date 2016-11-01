extern crate num;
use self::num::bigint;
use self::num::bigint::ToBigInt;

pub fn problem_16() {
    let mut big_number: bigint::BigInt = 2.to_bigint().unwrap();
    big_number = num::pow(big_number, 1000);
    let string_representation: String = big_number.to_str_radix(10_u32);
    let mut sum_of_digits = 0;
    for c in string_representation.chars() {
        sum_of_digits += c as i32 - 48;
    }

    println!("Sum of digits: {}", sum_of_digits);
}