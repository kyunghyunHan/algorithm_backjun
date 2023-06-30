use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input: String = String::new();
  reader.read_line(&mut input).unwrap();
let n:usize= input.trim().parse().unwrap();
  

  let characters = "abcdefgh".chars().collect::<Vec<char>>();
  let character = characters[(n - 1)  % 8];
  let number = (n + 7) / 8;

  writeln!(writer,"{}{}", character, number).unwrap();




}