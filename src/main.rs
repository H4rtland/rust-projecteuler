extern crate time;
use time::PreciseTime;

mod problem_18;

fn main() {
    let start_time = PreciseTime::now();
    problem_18::problem_18();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}