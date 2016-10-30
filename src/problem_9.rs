pub fn problem_9() {
    'forloop: for i in 1..1000 {
        for j in i..1000 {
            let k = 1000-i-j;
            if i*i + j*j == k*k {
                println!("{} + {} = {}, abc={}", i, j, k, i*j*k);
                break 'forloop;
            }
        }
    }
}