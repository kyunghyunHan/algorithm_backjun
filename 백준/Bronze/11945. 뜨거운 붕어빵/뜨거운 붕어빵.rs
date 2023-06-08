use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};
use std::cmp;
fn main() {
 let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut result= 0;
let mut input = String::new();
reader.read_line(&mut input).unwrap();
let nums: Vec<isize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<isize>>();
 

for i in 0..nums[0]{
  input.clear();
  reader.read_line(&mut input).unwrap();
  let mut nums2= input.trim();

  for i in nums2.chars().rev(){
    write!(writer,"{}",i).unwrap();
  }
  writeln!(writer,"").unwrap();
 

}

}
