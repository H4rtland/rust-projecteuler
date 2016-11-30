extern crate time;
use time::PreciseTime;

mod problem_23;

fn main() {
    let start_time = PreciseTime::now();
    problem_23::problem_23();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}