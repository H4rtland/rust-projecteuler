extern crate time;
use time::PreciseTime;

mod problem_12;

fn main() {
    let start_time = PreciseTime::now();
    problem_12::problem_12();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}