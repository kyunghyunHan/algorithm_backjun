use std::io::{self, BufRead};
use std::cmp::Ordering;


fn main() {
    let stdin = io::stdin();
    let mut word: Vec<String> = vec![];

    // N 입력받기
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

  println!("{}",n*(n-1));

   
}