extern crate time;
use time::PreciseTime;

mod problem_25;

fn main() {
    let start_time = PreciseTime::now();
    problem_25::problem_25();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}