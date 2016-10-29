fn is_prime(n: i64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn problem_3() {
    let mut big_number: i64 = 600851475143;
    for i in 2..big_number {
        if i >= big_number {
            break;
        }
        if big_number % i == 0 {
            big_number = big_number / i;
            println!("{}", i);
        }
    }
    println!("Largest factor is {}", big_number);
}