use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    match input.next() {
        Some(Ok(line)) => {
            let mut v: Vec<usize> = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let n = v[0];
            let k = v[1];

            let mut dp = [[0usize; 31]; 31];

            dp[1][1] = 1;
            dp[2][1] = 1;
            dp[2][2] = 1;
            dp[3][1] = 1;
            dp[3][2] = 2;
            dp[3][3] = 1;

            for i in 4..=n {
                for j in 1..=n {
                    if j ==1 || i==j {
                        dp[i][j] = 1;
                    }

                    if j >=2 || j <n{
                        dp[i][j] = dp[i-1][j-1]+dp[i-1][j];
                    }
                }
            }

            writeln!(writer, "{}", dp[n][k]).unwrap();
        }
        _ => {}
    }
    writer.flush().unwrap();
}
