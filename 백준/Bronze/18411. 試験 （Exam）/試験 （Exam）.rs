use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::collections::HashMap;
use std::sync::Arc;
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input: String = String::new();
  reader.read_line(&mut input).unwrap();
  let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
  nums.sort();

  writeln!(writer,"{}",nums[2]+nums[1]).unwrap();
}