use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
use std::cmp::max;
use std::result;
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());

  let mut input= String::new();

  reader.read_line(&mut input).unwrap();

  let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());

  let a= nums.next().unwrap();
  let b= nums.next().unwrap();

 let result = max(a, b);

 writeln!(writer,"{}",result).unwrap();


}