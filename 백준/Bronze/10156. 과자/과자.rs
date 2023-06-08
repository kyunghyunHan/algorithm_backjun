use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};
use std::cmp;
fn main() {
 let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut result= 0;
let mut input = String::new();
reader.read_line(&mut input).unwrap();
let nums: Vec<isize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<isize>>();
 

 if nums[0]*nums[1]-nums[2]>0{
writeln!(writer,"{}",nums[0]*nums[1]-nums[2]).unwrap();
 }else{
  writeln!(writer,"{}","0").unwrap();

 }
}
