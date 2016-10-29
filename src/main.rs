extern crate time;
use time::PreciseTime;

mod problem_4;

fn main() {
    let start_time = PreciseTime::now();
    problem_4::problem_4();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}