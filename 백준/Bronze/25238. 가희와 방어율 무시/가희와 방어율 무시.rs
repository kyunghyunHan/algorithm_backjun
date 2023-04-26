use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input"); // 표준 입력에서 입력값을 읽음

    let mut input_iter = input.trim().split_whitespace();
    let a: f64 = input_iter.next().unwrap().parse().expect("Failed to parse input"); // 입력값을 실수로 변환
    let b: f64 = input_iter.next().unwrap().parse().expect("Failed to parse input");

    if a * (100.0 - b) / 100.0 >= 100.0 {
        println!("0");
    } else {
        println!("1");
    }
}