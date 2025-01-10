use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    match input.next() {
        Some(Ok(line)) => {
            let n = line.parse::<usize>().unwrap();
            let mut dp = [0; 1001];
            dp[1] = 1;
            dp[2] = 3;
            dp[3] = 5;
            for i in 4..=n {
                // dp[3] = dp[1] +dp[2]
                dp[i] = (dp[i - 2] * 2) + dp[i - 1];
                dp[i] %= 10007;
            }

            writeln!(writer, "{}", dp[n]).unwrap();
        }
        _ => {
            panic!("Error");
        }
    }
    writer.flush().unwrap();
}
