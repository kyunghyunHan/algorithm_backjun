use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        let mut dp = [0u64; 1001];

        //돌 n개가있어
        //돌은 1,3,4개를 가질수있음
        dp[1] = 1;
        dp[2] = 0;
        dp[3] = 1;
        dp[4] = 1;

        for i in 5..=n {
            if dp[i - 1] + dp[i - 3] + dp[i - 4] == 3 {
                dp[i] = 0;
            } else {
                dp[i] = 1;
            }
        }
        if dp[n] == 1 {
            writeln!(writer, "{}", "SK").unwrap();
        } else {
            writeln!(writer, "{}", "CY").unwrap();
        }
    }
}
