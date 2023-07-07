use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input= String::new();
let mut writer= BufWriter::new(stdout().lock());
reader.read_line(&mut input).unwrap();
let mut x :i32= input.trim().parse().unwrap();

input.clear();
reader.read_line(&mut input).unwrap();
let mut y :i32= input.trim().parse().unwrap();
input.clear();
reader.read_line(&mut input).unwrap();
let mut z :i32= input.trim().parse().unwrap();

if x+y <=z{
  writeln!(writer,"{}",1).unwrap();

}else {
  writeln!(writer,"{}",0).unwrap();
}

}