use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let nm = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let n = nm[0];
        let m = nm[1];
        let mut dp = [[0u64; 1001]; 1001];

        dp[0][0] = 1;
        dp[2][1] = dp[1][0] + dp[2][0];
        dp[2][2] = dp[1][2] + dp[2][1];

        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = (dp[i - 1][j] + dp[i][j - 1] + dp[i - 1][j - 1]) % 1_000_000_007;
            }
        }

        writeln!(writer, "{}", dp[n][m]).unwrap();
    }
}
