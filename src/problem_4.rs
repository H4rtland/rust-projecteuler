pub fn problem_4() {
    let mut largest_palindrome = 0;
    for i in 100..999 {
        for y in i..999 {
            let product = i*y;
            if product < largest_palindrome {
                break
            }
            let s = product.to_string();
            if s == s.chars().rev().collect::<String>() {
                largest_palindrome = product
            }
        }
    }
    println!("Largest palindrome: {}", largest_palindrome)
}