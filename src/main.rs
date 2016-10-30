extern crate time;
use time::PreciseTime;

mod problem_5;

fn main() {
    let start_time = PreciseTime::now();
    problem_5::problem_5();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}