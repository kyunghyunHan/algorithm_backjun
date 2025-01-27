use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        let mut dp = vec![1u64; 10_000_001];

        dp[1] = 1;

  
        for i in 2..=n {
            dp[i] = (dp[i - 1] + dp[i - 2]) % 10;
        }
        writeln!(writer, "{}", dp[n]).unwrap();
    }
}
