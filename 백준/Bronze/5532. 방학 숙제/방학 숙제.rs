use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};
use std::cmp;
fn main() {
 let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input: String = String::new();
  reader.read_line(&mut input).unwrap();
  let l: i32 = input.trim().parse().unwrap();


  input.clear();
  reader.read_line(&mut input).unwrap();
  let a: u32 = input.trim().parse().unwrap();

  input.clear();
  reader.read_line(&mut input).unwrap();
  let b: u32 = input.trim().parse().unwrap();
  input.clear();
  reader.read_line(&mut input).unwrap();
  let c: u32 = input.trim().parse().unwrap();
  input.clear();
  reader.read_line(&mut input).unwrap();
  let d: u32 = input.trim().parse().unwrap();

  let freeday = l - cmp::max(((a as f32 / c as f32).ceil()) as i32, ((b as f32 / d as f32).ceil()) as i32);

  println!("{}", freeday);

}
