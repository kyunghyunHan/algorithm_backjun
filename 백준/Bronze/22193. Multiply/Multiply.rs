use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
  let mut nums:Vec<u128>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
  input.clear();

  reader.read_line(&mut input).unwrap();

    let a:u128= input.trim().parse().unwrap();
     
     
     input.clear();
     reader.read_line(&mut input).unwrap();
     let b:u128= input.trim().parse().unwrap();
      
     
    
    writeln!(writer,"{}",a*b as u128).unwrap();
}