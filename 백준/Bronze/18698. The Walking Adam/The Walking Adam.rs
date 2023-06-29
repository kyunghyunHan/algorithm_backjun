use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input: String = String::new();
  reader.read_line(&mut input).unwrap();
  let n= input.trim().parse().unwrap();

  for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let s= input.trim();
    let mut count= 0;
    for i in s.chars(){
      if i=='U'{
        count+=1;

      }else{
        break;
      }
      
    }
  writeln!(writer,"{}",count).unwrap(); 
  }
}