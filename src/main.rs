extern crate time;
use time::PreciseTime;

mod problem_13;

fn main() {
    let start_time = PreciseTime::now();
    problem_13::problem_13();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}