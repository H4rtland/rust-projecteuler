fn letters_in(number: i32) -> i32 {
    match number {
        1 => 3,
        2 => 3,
        3 => 5,
        4 => 4,
        5 => 4,
        6 => 3,
        7 => 5,
        8 => 5,
        9 => 4,
        _ => 0,
    }
}

fn letters_in_10s(number: i32) -> i32 {
    // i.e 6 => len("sixty")
    match number {
        2 => 6,
        3 => 6,
        4 => 5,
        5 => 5,
        6 => 5,
        7 => 7,
        8 => 6,
        9 => 6,
        _ => 0,
    }
}

fn letters_in_number(number: i32) -> i32 {
    // special cases
    let special_case = match number {
        1000 => 11,
        10 => 3,
        11 => 6,
        12 => 6,
        13 => 8,
        14 => 8,
        15 => 7,
        16 => 7,
        17 => 9,
        18 => 8,
        19 => 8,
        _ => 0,
    };
    if special_case > 0 {
        println!("{}, {}", number, special_case);
        return special_case;
    }
    let mut letters = 0;
    let hundreds = (number - number%100) / 100;
    if hundreds > 0 {
        letters += letters_in(hundreds) + 7; // ___ hundred
        if number % 100 != 0 {
            letters += 3; // and
        }
    }
    let remaining = match number%100 {
        10 => 3,
        11 => 6,
        12 => 6,
        13 => 8,
        14 => 8,
        15 => 7,
        16 => 7,
        17 => 9,
        18 => 8,
        19 => 8,
        _ => 0,
    };
    letters += remaining;
    if remaining == 0 {
        let tens = (number - hundreds*100) / 10;
        letters += letters_in_10s(tens);
        let ones = number % 10;
        letters += letters_in(ones);
        println!("{}, {}, {}, {}, {}", number, hundreds, tens, ones, letters);
    } else {
        println!("{}, {}", number, letters);
    }
    return letters;


}

pub fn problem_17() {
    let mut total_letters = 0;
    for i in 1..1001 {
        total_letters += letters_in_number(i);
    }
    println!("Total letters in 1 to 1000: {}", total_letters);
}