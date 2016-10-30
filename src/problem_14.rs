fn next(i: i64) -> i64 {
    match i%2 == 0 {
        true => i/2,
        false => 3*i + 1
    }
}

pub fn problem_14() {
    let mut max_chain: i64 = 0;
    let mut max_at = 0;
    for i in 2..1000000 {
        let mut current_value = i as i64;
        let mut terms = 0;
        while current_value != 1 {
            current_value = next(current_value);
            terms += 1;
        }
        if terms > max_chain {
            max_chain = terms;
            max_at = i;
        }
    }
    println!("Max chain at {} ({} terms)", max_at, max_chain);
}