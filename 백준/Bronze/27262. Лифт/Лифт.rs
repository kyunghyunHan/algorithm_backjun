use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();
reader.read_line(&mut input).unwrap();

let mut nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

let n= nums[0];
let k = nums[1];
let a= nums[2];
let b= nums[3];

let elev_time= (1-k).abs() * b+(n-1)*b;
let start_time= (n-1)*a;

writeln!(writer,"{} {}",elev_time,start_time).unwrap();
}
