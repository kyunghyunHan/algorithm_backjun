use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut v:Vec<i32> = Vec::new();
    input.clear();
    stdin.read_line(&mut input).unwrap();
    for num in input.split_whitespace() {
        v.push(num.trim().parse().unwrap());
    }

    let mut dp = vec![0; n];
    let mut ans = 0;

    for i in 1..n - 1 {
        dp[i] = v[i] + v[i - 1].min(v[i + 1]);
        ans = ans.max(dp[i]);
    }
    ans = ans.max(v[0]);
    ans = ans.max(v[n - 1]);
    println!("{}", ans);
}
