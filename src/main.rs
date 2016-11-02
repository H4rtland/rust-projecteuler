extern crate time;
use time::PreciseTime;

mod problem_20;

fn main() {
    let start_time = PreciseTime::now();
    problem_20::problem_20();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}