use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut reader = reader.lines();

    match reader.next() {
        Some(Ok(line)) => {
            let n = line.parse::<usize>().unwrap();

            let mut dp = [0u64; 1000001];
            dp[1] = 1;
            dp[2] = 1;
            dp[3] = 2;
            dp[4] = 3;

            for i in 5..=n {
                dp[i] = (dp[i - 1] + dp[i - 2]) % 1_000_000_007;
            }

            writeln!(writer, "{}", dp[n]).unwrap();
        }
        _ => {}
    }
    writer.flush().unwrap();
}
