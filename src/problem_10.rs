pub fn problem_10() {
    let mut numbers: Vec<bool>;
    numbers = vec![true; 2000000];
    numbers[0] = false;
    numbers[1] = false;
    let mut i: i32 = 4;
    while i < 2000000 {
        numbers[i as usize] = false;
        i += 2;
    }
    let mut i: i32 = 3;
    while i < 2000000 {
        if numbers[i as usize] {
            let mut j: i64 = (i as i64)*(i as i64);
            while j < 2000000 {
                numbers[j as usize] = false;
                j += i as i64;
            }
        }
        i += 2;
    }
    let mut sum: i64 = 0;
    for i in 2..2000000 {
        if numbers[i as usize] == true {
            sum += i as i64;
        }
    }
    println!("Sum of primes below 2,000,000 is {}", sum);
}