extern crate time;
use time::PreciseTime;

mod problem_26;

fn main() {
    let start_time = PreciseTime::now();
    problem_26::problem_26();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}