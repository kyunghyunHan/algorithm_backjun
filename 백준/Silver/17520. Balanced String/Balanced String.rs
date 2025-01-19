use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut reaer = reader.lines();

    match reaer.next() {
        Some(Ok(line)) => {
            let n = line.parse::<usize>().unwrap();
            let mut dp = [0i64; 100_001];
            dp[1] = 2;

            for i in 2..=n {
                if i % 2 == 0 {
                    // 짝수인 경우
                    dp[i] = dp[i - 1];
                } else {
                    // 홀수인 경우
                    dp[i] = (dp[i - 1] * 2) % 16769023;
                }
            }

            writeln!(writer, "{}", dp[n]).unwrap();
        }
        _ => {}
    }
    writer.flush().unwrap();
}
