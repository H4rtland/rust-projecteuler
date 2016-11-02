fn d(n: i32) -> i32 {
    let mut divisors_sum = 0;
    for i in 1..(n/2)+1 {
        if n % i == 0 {
            divisors_sum += i;
        }
    }
    divisors_sum
}

pub fn problem_21() {
    let mut amicable_sum = 0;
    for i in 1..10000 {
        let divisors_sum = d(i);
        if divisors_sum == i {
            continue;
        }
        let reverse_amicable = d(divisors_sum);
        if reverse_amicable == i {
            amicable_sum += i;
        }
    }
    println!("Sum of amicable numbers under 10000: {}", amicable_sum);
}