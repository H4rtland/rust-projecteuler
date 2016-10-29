pub fn problem_2() {
    let mut prev_1 = 1;
    let mut prev_2 = 2;
    let mut even_sum = 2;
    loop {
        let next_term = prev_1 + prev_2;
        if next_term > 4000000 {
            break;
        }
        if next_term % 2 == 0 {
            even_sum += next_term;
        }
        prev_1 = prev_2;
        prev_2 = next_term;
    }
    println!("{}", even_sum);
}