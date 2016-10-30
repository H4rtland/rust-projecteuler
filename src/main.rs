extern crate time;
use time::PreciseTime;

mod problem_7;

fn main() {
    let start_time = PreciseTime::now();
    problem_7::problem_7();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}