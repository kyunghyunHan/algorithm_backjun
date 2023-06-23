use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let a:i32= input.trim().parse().unwrap();
     
     
     input.clear();
     reader.read_line(&mut input).unwrap();
     let b:i32= input.trim().parse().unwrap();
      
     
    
    writeln!(writer,"{}",b-a).unwrap();
}