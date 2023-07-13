use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();
reader.read_line(&mut input).unwrap();

let mut nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
let mut n= nums[0];
let mut m= nums[1];
let result = if n < 8 { m + 7 } else { n - 7 };
writeln!(writer,"{}",result).unwrap();
}
