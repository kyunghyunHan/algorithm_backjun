use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let n: i32 = lines.next().unwrap().trim().parse().unwrap();

    for (i, line) in lines.take(n as usize).enumerate() {
        println!("{}. {}", i + 1, line);
    }
}
