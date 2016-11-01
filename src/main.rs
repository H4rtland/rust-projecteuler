extern crate time;
use time::PreciseTime;

mod problem_67;

fn main() {
    let start_time = PreciseTime::now();
    problem_67::problem_67();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}