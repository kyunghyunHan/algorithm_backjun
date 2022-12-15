//피보나치 수5
use std::io;
use std::str;
fn main() {
    let mut input_one = String::new();
    io::stdin().read_line(&mut input_one).unwrap();
    let v1: Vec<i32> = input_one
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let mut N = v1[0];
    println!("{}", fibo(N))
}
fn fibo(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fibo(n - 2) + fibo(n - 1);
}