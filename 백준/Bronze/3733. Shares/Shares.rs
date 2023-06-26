use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let n = numbers[0];
        let s = numbers[1];

        let max_x = s / (n+1);
        println!("{}", max_x);
    }
}
