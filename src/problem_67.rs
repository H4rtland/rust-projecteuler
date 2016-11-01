use std::cmp;

use std::fs::File;
use std::io::Read;


pub fn problem_67() {
    let mut triangle = String::new();
    let mut f = File::open("p067_triangle.txt").expect("Unable to open file");
    f.read_to_string(&mut triangle).expect("Unable to read string");

    let rows_str: Vec<&str> = triangle.split("\n").collect();

    let mut rows = Vec::new();

    for row in rows_str {
        if row.len() == 0 {
            continue;
        }
        let row_values: Vec<i32> = row.split(" ").map(|x| x.parse().unwrap()).collect();
        rows.push(row_values);
    }

    while rows.len() > 1 {
        for i in 0..rows[rows.len()-1].len()-1 {
            let last_row = rows.len()-1;
            rows[last_row-1][i] += cmp::max(rows[last_row][i], rows[last_row][i+1]);
        }
        let last_row = rows.len()-1;
        rows.truncate(last_row);
    }
    println!("{}", rows[0][0]);
}