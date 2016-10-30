use std::cmp;

pub fn problem_12() {

    let mut primes: Vec<i32> = Vec::new();
    'outer: for i in 2..65500 {
        for j in &primes {
            if i % j == 0 {
                continue 'outer;
            }
        }
        primes.push(i);
    }

    let mut max_divisors = 0;
    let mut index = 0;
    let mut triangle_number = 0;

    while max_divisors < 500 {
        index += 1;
        triangle_number += index;
        let mut exponents = Vec::new();
        for prime in &primes {
            if prime > &triangle_number {break}
            if triangle_number % prime == 0 {
                let mut exponent = 0;
                let mut test_num = triangle_number;
                while test_num % prime == 0 {
                    test_num /= *prime as i32;
                    exponent += 1;
                }
                exponents.push(exponent);
            }
        }
        let mut divisors = 1;
        for exp in exponents{
            divisors = divisors*(exp+1);
        }
        max_divisors = cmp::max(divisors, max_divisors);
    }
    println!("First triangle number with >500 divisors: {} ({})", triangle_number, max_divisors);
}