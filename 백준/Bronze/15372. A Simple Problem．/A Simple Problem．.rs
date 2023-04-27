use std::io::{stdin,stdout,BufReader,BufRead,BufWriter,Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let impit:i128= input.trim().parse().unwrap();
 
    for _ in 0..impit {
     input.clear();
     reader.read_line(&mut input).unwrap();
     let impit:i128= input.trim().parse().unwrap();
  
     writeln!(writer,"{}",i128::pow(impit, 2)).unwrap();
    }

 
}
