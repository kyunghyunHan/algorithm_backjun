use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();
reader.read_line(&mut input).unwrap();
let mut nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

let result= i32::max(nums[1],nums[0]-nums[1])* i32::max(nums[2],nums[0]-nums[2]);

writeln!(writer,"{}",result*4).unwrap();
}
