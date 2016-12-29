extern crate time;
use time::PreciseTime;

mod problem_24;

fn main() {
    let start_time = PreciseTime::now();
    problem_24::problem_24();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}