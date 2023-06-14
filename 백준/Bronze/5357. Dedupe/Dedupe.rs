use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .expect("Failed to read input");

    for _ in 0..n {
        let input: String = lines.next().unwrap().unwrap();
        let mut ans = input.chars().next().unwrap().to_string();

        for (prev, curr) in input.chars().zip(input.chars().skip(1)) {
            if curr != prev {
                ans.push(curr);
            }
        }

        println!("{}", ans);
    }
}