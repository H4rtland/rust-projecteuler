extern crate time;
use time::PreciseTime;

mod problem_15;

fn main() {
    let start_time = PreciseTime::now();
    problem_15::problem_15();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}