extern crate time;
use time::PreciseTime;

mod problem_22;

fn main() {
    let start_time = PreciseTime::now();
    problem_22::problem_22();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}