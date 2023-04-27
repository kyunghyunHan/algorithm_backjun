use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let test_case: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..test_case {
        let problems: usize = lines.next().unwrap().parse().unwrap();
        for _ in 0..problems {
            let numbers: Vec<i32> = lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            println!("{} {}", numbers[0] + numbers[1], numbers[0] * numbers[1]);
        }
    }
}
