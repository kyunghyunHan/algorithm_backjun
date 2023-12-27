 use std::io::{prelude::*, BufReader, stdin, BufWriter, stdout};

 fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let ab:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
     let a= ab[0];
     let b= ab[1];

    if a>b{
        writeln!(writer,"{}",b+b+1).unwrap();
    }else{
        writeln!(writer,"{}",a+a-1).unwrap();
    }
    writer.flush().unwrap();
 }