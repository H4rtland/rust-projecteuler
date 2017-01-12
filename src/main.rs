extern crate time;
use time::PreciseTime;

mod problem_29;

fn main() {
    let start_time = PreciseTime::now();
    problem_29::problem_29();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}