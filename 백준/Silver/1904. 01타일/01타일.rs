use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    stdin.lock().read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();

    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    if n > 1 {
        dp[2] = 2;
    }

    for i in 3..=n {
        dp[i] = (dp[i - 1] + dp[i - 2]) % 15746;
    }

    println!("{}", dp[n]);
}
