extern crate time;
use time::PreciseTime;

mod problem_6;

fn main() {
    let start_time = PreciseTime::now();
    problem_6::problem_6();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}