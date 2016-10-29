extern crate time;
use time::PreciseTime;

mod problem_3;

fn main() {
    let start_time = PreciseTime::now();
    problem_3::problem_3();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}