use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let n: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    let mut dp: Vec<i32> = vec![1; n];

    for i in 0..n {
        for j in 0..i {
            if nums[i] > nums[j] && dp[i] < dp[j] + 1 {
                dp[i] = dp[j] + 1;
            }
        }
    }

    let max_length = dp.into_iter().max().unwrap_or(0);
    println!("{}", max_length);
}