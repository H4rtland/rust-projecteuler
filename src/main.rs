extern crate time;
use time::PreciseTime;

mod problem_16;

fn main() {
    let start_time = PreciseTime::now();
    problem_16::problem_16();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}