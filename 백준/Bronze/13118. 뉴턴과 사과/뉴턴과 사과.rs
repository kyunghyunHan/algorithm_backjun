use std::io::{BufRead,BufWriter,BufReader,Write,stdin,stdout};

fn main(){

  let mut reader= BufReader::new(stdin().lock());

let mut writer= BufWriter::new(stdout().lock());
 
let mut ans = 0;

let mut input = String::new();
reader.read_line(&mut input).unwrap();

let mut  p = input.trim().split_whitespace().map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();

input.clear();
reader.read_line(&mut input).unwrap();
let mut  nums = input.trim().split_whitespace().map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();




 for i in 0..4{
  if nums[0]==p[i] {
    ans=i+1;
  }
 }
 println!("{}",ans);
}