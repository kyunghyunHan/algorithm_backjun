use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    match input.next() {
        Some(Ok(line)) => {
            let n = line.parse::<usize>().unwrap();
            for _ in 0..n {
                match input.next() {
                    Some(Ok(line)) => {
                        let n2 = line.parse::<usize>().unwrap();
                        let mut dp = [0; 46];
                        dp[0] = 1;
                        dp[1] = 1;
                        for i in 2..=n2 {
                            dp[i] = dp[i - 1] + dp[i - 2];
                        }
                        writeln!(writer, "{}", dp[n2]).unwrap();
                    }
                    _=>{}
                }
            }
        }
        _ => {}
    }
    writer.flush().unwrap();
}
