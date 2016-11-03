use std::fs::File;
use std::io::Read;

fn name_score(name_ref: &str) -> i64 {
    let name = String::from(name_ref);
    let mut score = 0;
    for char in name.chars() {
        if char == '"' { continue }
        score += char as i64 - 64;
    }
    score
}

pub fn problem_22() {
    let mut names_string = String::new();
    let mut f = File::open("p022_names.txt").expect("Unable to open file");
    f.read_to_string(&mut names_string).expect("Unable to read string");

    let mut names_list: Vec<String> = Vec::new();

    for name in names_string.split(",") {
        names_list.push(String::from(name));
    }

    names_list.sort();

    let mut total_score: i64 = 0;

    for i in 0..names_list.len() {
        total_score += name_score(&names_list[i]) * ((i as i64)+1);
    }

    println!("Total names score: {}", total_score);
}