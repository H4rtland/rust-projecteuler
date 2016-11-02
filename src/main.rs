extern crate time;
use time::PreciseTime;

mod problem_21;

fn main() {
    let start_time = PreciseTime::now();
    problem_21::problem_21();
    let end_time = PreciseTime::now();
    println!("Execution took {} seconds", start_time.to(end_time))
}