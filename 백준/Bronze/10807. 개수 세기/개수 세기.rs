use std::fmt::Write;
use std::io;

fn main() {
    let mut input_a = String::new();
    let v: Vec<i32> = input_a
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    io::stdin().read_line(&mut input_a).unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v1: Vec<i32> = input
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).unwrap();
    let v2: Vec<i32> = input_c
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let mut count = 0;
    for i in v1 {
        if i == v2[0] {
            count += 1;
        }
    }
    println!("{:?}", count);
}
