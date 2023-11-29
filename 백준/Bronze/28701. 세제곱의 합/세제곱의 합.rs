use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let n: u64 = lines.next().unwrap().trim().parse().unwrap();

    let sum_n: u64 = (1..=n).sum();
    let square_sum_n = sum_n * sum_n;
    let sum_cube_n: u64 = (1..=n).map(|x| x * x * x).sum();

    println!("{}", sum_n);
    println!("{}", square_sum_n);
    println!("{}", sum_cube_n);
}
