use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let r: u64 = iter.next().unwrap().parse().unwrap();
    let c: u64 = iter.next().unwrap().parse().unwrap();
    let n: u64 = iter.next().unwrap().parse().unwrap();

    let rows = (r + n - 1) / n;
    let cols = (c + n - 1) / n;
    let total_cctv = rows * cols;

    println!("{}", total_cctv);
}
