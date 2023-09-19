//x보다 작은수
//10 5 //1423
use std::fmt::Write;
use std::io;
fn main() {
    //스트링
    let mut input = String::new();
    let mut output = String::new();
    let mut numbers_arry = String::new();
    //입력값받고
    io::stdin().read_line(&mut input).unwrap();
    io::stdin().read_line(&mut numbers_arry).unwrap();
    //배열선언
    let v1: Vec<i32> = input
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let v2: Vec<i32> = numbers_arry
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let mut output = String::new();
    for i in &v2 {
        if i < &v1[1] {
            print!("{} ", i);
        }
    }
}