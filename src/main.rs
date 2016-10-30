extern crate time;
use time::PreciseTime;

mod problem_14;

fn main() {
    let start_time = PreciseTime::now();
    problem_14::problem_14();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}