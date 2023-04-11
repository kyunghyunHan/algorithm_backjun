use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let nm: Vec<usize> = lines.next().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = nm[0];
    let m = nm[1];

    let mut set = HashSet::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        set.insert(line.trim().to_string());
    }

    let mut count = 0;
    for _ in 0..m {
        let line = lines.next().unwrap();
        if set.contains(line.trim()) {
            count += 1;
        }
    }

    println!("{}", count);
}