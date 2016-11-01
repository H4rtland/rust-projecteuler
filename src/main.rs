extern crate time;
use time::PreciseTime;

mod problem_17;

fn main() {
    let start_time = PreciseTime::now();
    problem_17::problem_17();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}