use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let t = line.parse::<usize>().unwrap();

        let mut dp = [0u64; 1_000_001];
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 4;

        for i in 4..=1_000_000{
            dp[i] = (dp[i - 3] + dp[i - 2] + dp[i - 1]) % 1_000_000_009;
        }

        for _ in 0..t {
            if let Some(Ok(line)) = input.next() {
                let g = line.parse::<usize>().unwrap();
                writeln!(writer,"{}",dp[g]).unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
