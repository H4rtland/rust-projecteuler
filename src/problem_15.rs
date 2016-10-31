pub fn problem_15() {
    let mut grid = Vec::new();

    let width = 20;
    let height = 20;

    // width+1 because width squares in grid but the last square has an extra vertex
    for _ in 0..width+1 {
        let v: Vec<i64>;
        v = vec![0; height+1];
        grid.push(v);
    }

    for i in 0..width+1 {
        grid[i][0] = 1;
    }
    for j in 0..height+1 {
        grid[0][j] = 1;
    }

    for i in 1..width+1 {
        for j in 1..height+1 {
            grid[i][j] = grid[i-1][j] + grid[i][j-1];
        }
    }

    println!("Total paths through grid: {}", grid[width][height]);
}