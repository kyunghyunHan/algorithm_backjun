use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input = String::new();
  reader.read_line(&mut input).unwrap();
  let n= input.trim().parse().unwrap();
  let mut v= Vec::new();
  for _ in 0..n{
      input.clear();
      reader.read_line(&mut input).unwrap();
      let n:u64= input.trim().parse().unwrap();
      if n==0 {
        v.pop();
      }else{
        v.push(n);
      }
  }
let result:u64= v.iter().sum();
  writeln!(writer,"{}",result).unwrap();
}