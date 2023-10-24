use std::{fmt::Write, io};
//달팽이는 올라가고 싶다.

fn main() {
    let mut Input_array = String::new();
    io::stdin().read_line(&mut Input_array).unwrap();
    let v1: Vec<i32> = Input_array
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let mut Day = 1;

    Day += (v1[2] - v1[0]) / (v1[0] - v1[1]);
    if (v1[2] - v1[0]) % (v1[0] - v1[1]) != 0 {
        Day = Day + 1;
    }
    if v1[0] >= v1[2] {
        println!("{}", 1)
    } else {
        println!("{}", Day);
    }
}
