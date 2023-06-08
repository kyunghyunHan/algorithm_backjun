use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};
use std::cmp;
fn main() {
 let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut result= 0;

for i in 0..2{
  let mut input = String::new();
  reader.read_line(&mut input).unwrap();
  let nums= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();
   
  let sum: usize = nums.iter().sum();
if result<sum{
  result= sum;
}
}
  
writeln!(writer,"{}",result).unwrap();
}
