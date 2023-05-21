use core::num;
use std::io::{BufRead,BufWriter,BufReader,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());

    let mut input= String::new();

    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());

    let n= nums.next().unwrap();
    let s= nums.next().unwrap();

    println!("{}",usize::min(n/2, s));
}