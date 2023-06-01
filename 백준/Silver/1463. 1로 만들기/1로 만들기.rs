use std::cmp;

fn main() {
    let mut dp = vec![0; 1000001];
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    for i in 2..=n {
        // 1. Subtract 1
        dp[i] = dp[i - 1] + 1;

        // 2. Divide by 2 if divisible
        if i % 2 == 0 {
            dp[i] = cmp::min(dp[i], dp[i / 2] + 1);
        }

        // 3. Divide by 3 if divisible
        if i % 3 == 0 {
            dp[i] = cmp::min(dp[i], dp[i / 3] + 1);
        }
    }

    println!("{}", dp[n]);
}
