use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut reeader = reader.lines();

    match reeader.next() {
        Some(Ok(line)) => {
            let rcw = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();
            let r = rcw[0];
            let c = rcw[1];
            let w = rcw[2];

            let mut dp = [[0u64; 31]; 31];

            dp[1][1] = 1;
            dp[2][1] = 1;
            dp[2][2] = 1;
            dp[3][1] = 1;
            dp[3][2] = 2;
            dp[3][3] = 1;

  

            for i in 4..=30 {
                for j in 1..=i {
                    if j == 1 || j == i {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                    }                }
            }
            let mut result = 0;

            for i in 0..w {
                for j in 0..=i {
                    result += dp[r + i][c + j];
                }
            }
            writeln!(writer,"{}",result).unwrap();
        }
        _ => {}
    }
}
