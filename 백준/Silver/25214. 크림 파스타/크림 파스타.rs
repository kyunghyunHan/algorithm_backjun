use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut wirter = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        if let Some(Ok(line)) = input.next() {
            let v = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let mut dp = [0u64; 200_001];
            let mut mins = v[0];
            dp[0] = 0;

            for i in 1..=n {
                if mins > v[i - 1] {
                    mins = v[i - 1];
                }
                if dp[i - 1] < v[i - 1] - mins {
                    dp[i] = v[i - 1] - mins;
                } else {
                    dp[i] = dp[i - 1];
                }
            }

            for i in 1..=n {
                write!(wirter, "{} ", dp[i]).unwrap();
            }
        }
    }
    wirter.flush().unwrap();
}
