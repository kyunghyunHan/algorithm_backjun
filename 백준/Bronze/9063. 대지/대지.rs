use std::io::{self, Read};
use std::cmp::{max, min};

const MAX: i64 = 0x3f3f3f3f;
type ll = i64;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();

    let n: ll = iter.next().unwrap().parse().unwrap();
    let (mut minX, mut maxX, mut minY, mut maxY) = (MAX, -MAX, MAX, -MAX);

    for _ in 0..n {
        let x: ll = iter.next().unwrap().parse().unwrap();
        let y: ll = iter.next().unwrap().parse().unwrap();
        minX = min(x, minX);
        maxX = max(x, maxX);
        minY = min(y, minY);
        maxY = max(y, maxY);
    }

    println!("{}", (maxX - minX) * (maxY - minY));
}