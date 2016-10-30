pub fn problem_5() {
    let mut i: i64 = 20;
    'main: loop {
        i += 1;
        for j in 1..20 {
            if i % j > 0 {
                continue 'main;
            }
        }
        break;
    }
    println!("Largest number is {}", i);
}