pub fn problem_6() {
    let n = 100.0;
    let sum_squares = (n/3_f32)*(n+1_f32)*(n+0.5);
    let square_sums = ((n/2_f32)*(n+1_f32))*((n/2_f32)*(n+1_f32));
    let difference = square_sums - sum_squares;
    println!("Difference is {}", difference)
}