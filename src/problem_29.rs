fn factors_of(mut n: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new();
    'outer: loop {
        for i in 2..(n/2)+1 {
            if n % i == 0 {
                factors.push(i);
                n = n/i;
                continue 'outer;
            }
        }
        break;
    }
    factors.push(n);
    factors
}

pub fn problem_29() {
    let mut seen_factor_signatures: Vec<Vec<i32>> = Vec::new();
    for a in 2..101 {
        for b in 2..101 {
            let mut factors: Vec<i32> = Vec::new();
            let f = factors_of(a);
            for _ in 0..b {
                factors.extend(f.iter().cloned());
            }
            factors.sort();
            if !seen_factor_signatures.contains(&factors) {
                seen_factor_signatures.push(factors);
            }
        }
    }
    println!("{}", seen_factor_signatures.len());
}