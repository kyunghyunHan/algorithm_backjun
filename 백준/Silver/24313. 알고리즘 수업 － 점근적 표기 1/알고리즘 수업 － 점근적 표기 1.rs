use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let input: Vec<i32> = input.trim().split(" ")
                               .map(|s| s.parse().expect("parse error"))
                               .collect();
    let a1 = input[0];
    let a0 = input[1];

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let c: i32 = input.trim().parse().expect("parse error");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let n: i32 = input.trim().parse().expect("parse error");

    if a1 * n + a0 <= c * n && c >= a1 {
        println!("1");
    } else {
        println!("0");
    }
}