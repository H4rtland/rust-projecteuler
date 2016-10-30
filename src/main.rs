extern crate time;
use time::PreciseTime;

mod problem_10;

fn main() {
    let start_time = PreciseTime::now();
    problem_10::problem_10();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}