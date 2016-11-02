extern crate time;
use time::PreciseTime;

mod problem_19;

fn main() {
    let start_time = PreciseTime::now();
    problem_19::problem_19();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}