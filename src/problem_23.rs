use std::collections::HashMap;

fn is_abundant(n: i32, cache: &mut HashMap<i32, bool>) -> bool {
    if cache.contains_key(&n) {
        return match cache.get(&n) {
            Some(value) => *value, // * dereferences value from cache
            None => false
        };
    }
    let mut divisor_sum: i32 = 0;
    for i in 1..n {
        if n % i == 0 {
            divisor_sum += i;
            if divisor_sum > n {
                cache.insert(n, true);
                return true;
            }
        }
    }
    cache.insert(n, false);
    return false;
}

pub fn problem_23() {
    let mut cached_values = HashMap::new();
    let mut non_abundant_sum: i32 = 0;
    'outer: for i in 1..28123+1 {
        if i % 1000 == 0 {
            println!("{}", i);
        }
        for n1 in 12..i {
            if is_abundant(n1, &mut cached_values) && is_abundant(i-n1, &mut cached_values) {
                continue 'outer;
            }
        }
        non_abundant_sum += i;
    }
    println!("Non abundant sum: {}", non_abundant_sum);
}