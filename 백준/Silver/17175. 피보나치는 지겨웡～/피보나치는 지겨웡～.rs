use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();
    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();
        let mut dp = [0u64; 51];
        dp[0]= 1;
        dp[1] = 1;
        dp[2] = 3; //2
        dp[3] = 5; //2

        for i in 4..=n {
            dp[i] = ( 1+dp[i - 1] + dp[i - 2] ) % 1_000_000_007;
        }
        writeln!(writer,"{}",dp[n]).unwrap();
    }
    writer.flush().unwrap();
}
