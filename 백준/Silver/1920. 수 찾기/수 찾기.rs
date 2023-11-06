use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::collections::HashSet;

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:i64= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let a: HashSet<i64> = input
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();    
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m:i64= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let v= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i64>>();
    
    for &num in &v {
        if a.contains(&num) {
            writeln!(writer,"1").unwrap()
        } else {
            writeln!(writer,"0").unwrap()
        }
    }

  writer.flush().unwrap();
}