use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){

    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
   let mut n :u128= input.trim().parse().unwrap();
  n-=1;
   writeln!(writer,"{}",n/5+1).unwrap();


}
