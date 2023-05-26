use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
let mut reader= BufReader::new(stdin().lock());

let mut input= String::new();
reader.read_line(&mut input).unwrap();

let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

if nums[0]*nums[2]%(nums[1] * nums[3] * 2)!=0{
  println!("{}",0);
}else{
    println!("{}",1);
}
}