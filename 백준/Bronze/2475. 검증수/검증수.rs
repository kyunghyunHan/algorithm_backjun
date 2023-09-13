use std::io::{BufRead,BufReader,Write,BufWriter,stdin,stdout};
fn main(){
   let mut reader= BufReader::new(stdin().lock());
   let mut writer = BufWriter::new(stdout().lock());
   let mut input = String::new();
   reader.read_line(&mut input).unwrap();
   let mut nums= input.trim().split_whitespace().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
   let result= (i32::pow(nums[0],2)+i32::pow(nums[1],2)+i32::pow(nums[2],2)+i32::pow(nums[3],2)+i32::pow(nums[4],2))%10;
   writeln!(writer,"{}",result).unwrap();
 }
