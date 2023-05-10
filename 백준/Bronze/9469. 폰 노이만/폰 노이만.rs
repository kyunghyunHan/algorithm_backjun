use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();
    let p: usize = buffer.trim().parse().unwrap();
    for _ in 0..p {
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        let n: f64 = iter.next().unwrap().parse().unwrap();
        let d: f64 = iter.next().unwrap().parse().unwrap();
        let a: f64 = iter.next().unwrap().parse().unwrap();
        let b: f64 = iter.next().unwrap().parse().unwrap();
        let f: f64 = iter.next().unwrap().parse().unwrap();
        let time_left = d / (a + b);
        println!("{} {:.6}", n, f * time_left);
    }
}
