use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());

  let mut input = String::new();
  reader.read_line(&mut input).unwrap();
  let mut n:u64= input.trim().parse().unwrap();
  /*
  1^3+2^3+3^3+4^3
   */
let mut result = 0;
  for i in 1..=n{
  result +=u64::pow(i, 3);
  }


writeln!(writer,"{}",result).unwrap();
   
}