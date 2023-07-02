use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut input = String::new();
  let mut writer= BufWriter::new(stdout().lock());
  reader.read_line(&mut input).unwrap();

  let mut p:usize=input.trim().parse().unwrap();
   
  input.clear();
  reader.read_line(&mut input).unwrap();

  let mut q: usize= input.trim().parse().unwrap();
 let mut result= "";
  if p <= 50 && q <= 10{
    result=  "White";
  }else if q > 30{
    result=  "Red";
  }else {
    result=  "Yellow";
  } 
  writeln!(writer,"{}",result).unwrap();
}
