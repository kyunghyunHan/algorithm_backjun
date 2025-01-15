use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut reader = reader.lines();

    match reader.next() {
        Some(Ok(line)) => {
            let n = line.parse::<usize>().unwrap();

            let mut dp = [0u64; 36];
            dp[0] = 1;
            dp[1] = 1;
            dp[2] = 2;
            dp[3] = 5;

            for i in 4..=n {
                // dp[i] = dp[i - 1] + dp[n - 2] + dp[n - 1];

                for  j in 0..i {
                    // 문제의 점화식 그대로 이용
                    dp[i] += dp[j] * dp[i - j - 1];
                    //dp[4] = dp[0] * dp[3]
                }
            }

            writeln!(writer, "{}", dp[n]).unwrap();
        }
        _ => {}
    }
    writer.flush().unwrap();
}
