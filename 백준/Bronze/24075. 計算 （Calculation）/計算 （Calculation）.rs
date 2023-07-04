use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
  let mut writer= BufWriter::new(stdout().lock());
  let mut nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

  let a=i32::max(nums[0]-nums[1] ,nums[0]+nums[1]);

  let b=i32::min(nums[0]-nums[1] ,nums[0]+nums[1]);

  writeln!(writer,"{}\n{}",a,b).unwrap();
}