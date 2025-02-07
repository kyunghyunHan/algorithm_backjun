use std::cmp;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();
        if let Some(Ok(line)) = input.next() {
            let mut heights = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            heights.sort_by(|a, b| a.cmp(b));

            let mut dp = vec![n; n + 1];
            dp[0] = 0;

            for i in 0..n {
                dp[i + 1] = dp[i] + 1;

                if i > 0 && heights[i] - heights[i - 1] <= 20 {
                    dp[i + 1] = dp[i + 1].min(dp[i - 1] + 1);
                }
                if i > 1 && heights[i] - heights[i - 2] <= 10 {
                    dp[i + 1] = dp[i + 1].min(dp[i - 2] + 1);
                }
            }
            writeln!(writer, "{}", dp[n]).unwrap();
        }
    }
    writer.flush().unwrap();
}
