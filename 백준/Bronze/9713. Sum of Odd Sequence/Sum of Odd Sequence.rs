use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let q: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..q {
        let n: i32 = lines.next().unwrap().trim().parse().unwrap();
        let n = (n + 1) / 2;
        println!("{}", n * n);
    }
}
