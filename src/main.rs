extern crate time;
use time::PreciseTime;

mod problem_11;

fn main() {
    let start_time = PreciseTime::now();
    problem_11::problem_11();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}