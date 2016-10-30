extern crate time;
use time::PreciseTime;

mod problem_8;

fn main() {
    let start_time = PreciseTime::now();
    problem_8::problem_8();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}