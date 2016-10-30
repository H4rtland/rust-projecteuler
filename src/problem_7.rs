fn is_prime(n: i64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn problem_7() {
    let mut i: i64 = 2;
    let mut primes_found = 1;
    loop {
        i += 1;
        if i % 10000 == 0 {
            println!("{}, {}", i, primes_found)
        }
        if is_prime(i) {
            primes_found += 1;
        }
        if primes_found == 10001 {
            break;
        }
    }
    println!("10,001st prime number is {}", i);
}