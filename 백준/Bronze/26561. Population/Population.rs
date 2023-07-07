use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input= String::new();
let mut writer= BufWriter::new(stdout().lock());
reader.read_line(&mut input).unwrap();
let mut n:i32= input.trim().parse().unwrap();
for i in 0..n{
  input.clear();
  reader.read_line(&mut input).unwrap();
   let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
   writeln!(writer,"{}",nums[0]+nums[1]/4-nums[1]/7).unwrap();

}


}