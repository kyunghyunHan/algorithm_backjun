use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();
    input.read_line(&mut buffer)?;
    let n: usize = buffer.trim().parse().unwrap();
    let mut graph: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        buffer.clear();
        input.read_line(&mut buffer)?;
        let row: Vec<usize> = buffer
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        graph.push(row);
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n]; n];
    dp[0][0] = 1; // Initial value

    // Loop through the coordinates of the graph that can be reached
    for i in 0..n {
        for j in 0..n {
            // If the current coordinate is the bottom-right corner, stop the loop
            if i == n - 1 && j == n - 1 {
                println!("{}", dp[i][j]);
                return Ok(());
            }

            // Move to the right
            if j + graph[i][j] < n {
                dp[i][j + graph[i][j]] += dp[i][j];
            }

            // Move down
            if i + graph[i][j] < n {
                dp[i + graph[i][j]][j] += dp[i][j];
            }
        }
    }
    Ok(())
}
