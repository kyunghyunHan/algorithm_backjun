use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut input = String::new();
  let mut writer= BufWriter::new(stdout().lock());
  reader.read_line(&mut input).unwrap();
  let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
  let a= nums[0];
  let b= nums[1];
  let c= nums[2];
  

  if a + b == c || a + c == b || b + c == a {
    writeln!(writer,"{}",1).unwrap();
   
} else if a * b == c || a * c == b || b * c == a {
  writeln!(writer,"{}",2).unwrap();
} else {
  writeln!(writer,"{}",3).unwrap();
}
}