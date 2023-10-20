use std::{fmt::Write, io};

fn main() {
    //시험을 본 과목
    let mut numbers_arry = String::new();
    io::stdin().read_line(&mut numbers_arry).unwrap();
    let numbers: Vec<&str> = numbers_arry.split_whitespace().collect();
    let N: i32 = numbers[0].parse::<i32>().unwrap();
    //점수 배열
    let mut numbers_arry = String::new();
    io::stdin().read_line(&mut numbers_arry).unwrap();
    let mut v1: Vec<i32> = numbers_arry
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let result1 = v1.iter().max();
    let mut result2 = 0;
    match result1 {
        Some(x) => result2 = *x,
        None => (),
    }
    let mean: f64 =
        v1.iter().map(|&val| val as f64).sum::<f64>() / result2 as f64 * 100 as f64 / N as f64;
    println!("{:?}", mean);
}
