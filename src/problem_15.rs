pub fn problem_15() {
    let mut grid = Vec::new();

    // 21 because 20 squares in grid but the last square has an extra vertex
    for _ in 0..21 {
        let v: Vec<i64>;
        v = vec![0; 21];
        grid.push(v);
    }

    for i in 0..21 {
        grid[i][0] = 1;
        grid[0][i] = 1;
    }

    for i in 1..21 {
        for j in 1..21 {
            grid[i][j] = grid[i-1][j] + grid[i][j-1];
        }
    }

    println!("Total paths through grid: {}", grid[20][20]);
}