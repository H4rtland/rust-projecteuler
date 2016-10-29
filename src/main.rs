extern crate time;
use time::PreciseTime;

mod problem_2;

fn main() {
    let start_time = PreciseTime::now();
    problem_2::problem_2();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}