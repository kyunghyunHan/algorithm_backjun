use std::io::{self, Read};
use std::cmp::min;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();

    let mut a = Vec::new();
    for _ in 0..3 {
        a.push(iter.next().unwrap().parse::<i32>().unwrap());
    }
    a.sort();

    if a[0] + a[1] > a[2] {
        println!("{}", a[0] + a[1] + a[2]);
    } else {
        println!("{}", 2 * (a[0] + a[1]) - 1);
    }
}