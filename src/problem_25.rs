extern crate num;
use self::num::bigint;
use self::num::bigint::ToBigUint;

pub fn problem_25() {
    let mut i: bigint::BigUint = 1.to_biguint().unwrap();
    let mut j: bigint::BigUint = 1.to_biguint().unwrap();
    let mut index = 2;
    loop {
        index += 1;
        let sum = i.clone() + j.clone();
        i = j;
        j = sum;
        if j.to_str_radix(10).len() == 1000 {
            println!("Index of first fib number with 1000 digits is {}", index);
            break;
        }
    }
}