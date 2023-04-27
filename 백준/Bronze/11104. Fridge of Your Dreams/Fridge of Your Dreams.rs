use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let n: i32 = lines.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let mut s = lines.next().unwrap().trim().to_string();
        s = s.chars().rev().collect();

        let mut ans = 0;
        let mut pow = 1;
        for c in s.chars() {
            if c == '1' {
                ans += pow;
            }
            pow *= 2;
        }

        println!("{}", ans);
    }
}
