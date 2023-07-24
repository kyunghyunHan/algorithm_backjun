use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();

    reader.read_line(&mut input).unwrap();
    let mut found_divisor = false;

    let mut nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let a= nums[0];
    let b= nums[1];

    let x1 = (a - 1) / 4 + 1;
    let y1 = (a - 1) % 4;
    let x2 = (b - 1) / 4 + 1;
    let y2 = (b - 1) % 4;
    println!("{}", (x2 - x1).abs() + (y2 - y1).abs());
}