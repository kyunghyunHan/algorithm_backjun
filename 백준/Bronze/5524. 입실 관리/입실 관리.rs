use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};
use std::cmp;
fn main() {
 let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input: String = String::new();
  reader.read_line(&mut input).unwrap();
  let n= input.trim().parse().unwrap();
  for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();

    let l = input.trim();
    writeln!(writer,"{}",l.to_lowercase()).unwrap();
    
  }
  

}
