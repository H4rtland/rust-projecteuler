use std::collections::HashMap;


fn recurring_length(n: i32) -> i32 {
    let mut remainder: i32 = 10;
    let mut seen: HashMap<i32, i32> = HashMap::new();
    let mut counter: i32 = 0;
    loop {
        if remainder == 0 {
            // divides evenly
            return 0;
        } else if seen.contains_key(&remainder) {
            return counter-seen.get(&remainder).unwrap();
        }
        seen.insert(remainder, counter);
        remainder = 10*(remainder % n);
        counter += 1;
    }
}

pub fn problem_26() {
    let mut max_number = 0;
    let mut max_recurrence = 0;
    for i in 2..1000 {
        let r = recurring_length(i);
        if r > max_recurrence {
            max_number = i as i32;
            max_recurrence = r;
        }
    }
    println!("max recurrence at number {}, {} repeating digits", max_number, max_recurrence);
}