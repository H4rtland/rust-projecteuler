extern crate chrono;

use self::chrono::*;
use std::ops::Add;

pub fn problem_19() {
    let mut sundays = 0;
    let mut date = UTC.ymd(1901, 1, 1);

    while date.year() < 2001 {
        date = date.add(Duration::days(1));
        if date.weekday() == Weekday::Sun && date.day() == 1 {
            sundays += 1;
        }
    }
    println!("Total Sundays on the first of a month: {}", sundays);
}