use std::io;

fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let change_str = input_a.chars().rev().collect::<String>();
    let v1: Vec<i32> = change_str
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    if v1[0] > v1[1] {
        println!("{}", v1[0]);
    } else {
        println!("{}", v1[1]);
    }
}
