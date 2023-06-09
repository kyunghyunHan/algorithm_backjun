use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};
use std::cmp;
fn main() {
 let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());

let mut input= String::new();



reader.read_line(&mut input).unwrap();

let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();


let x= nums[0];
let y= nums[1];


if x > y {
    write!(writer,"{}", x + y).unwrap();
  }
  else {
    write!(writer,"{}", y - x).unwrap();
  }
}
