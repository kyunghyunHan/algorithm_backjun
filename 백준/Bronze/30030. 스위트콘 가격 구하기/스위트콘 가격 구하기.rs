use std::{io::{BufReader,BufRead,BufWriter,Write,stdin,stdout}, array};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
  let b:f32= input.trim().parse().unwrap();

  
  writeln!(writer,"{}",b-b/11.0).unwrap();
}