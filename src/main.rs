extern crate time;
use time::PreciseTime;

mod problem_9;

fn main() {
    let start_time = PreciseTime::now();
    problem_9::problem_9();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}