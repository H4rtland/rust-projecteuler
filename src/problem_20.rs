extern crate num;
use self::num::bigint;
use self::num::bigint::ToBigInt;

fn factorial(n: i64) -> i64 {
    if n == 1 {
        1
    }
    n*factorial(n-1)
}

pub fn problem_20() {
    let mut big_number: bigint::BigInt = 1.to_bigint().unwrap();
    for i in 1..101 {
        big_number = big_number.checked_mul(&i.to_bigint().unwrap()).unwrap();
    }
    let string_representation: String = big_number.to_str_radix(10_u32);
    let mut sum_of_digits = 0;
    for c in string_representation.chars() {
        sum_of_digits += c as i32 - 48;
    }

    println!("Sum of digits: {}", sum_of_digits);
}