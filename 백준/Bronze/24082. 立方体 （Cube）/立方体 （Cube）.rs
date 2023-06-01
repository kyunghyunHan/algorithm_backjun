use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input = String::new();

reader.read_line(&mut input).unwrap();
let mut writer= BufWriter::new(stdout().lock());

let s:i32= input.trim().parse().unwrap();

writeln!(writer,"{}",i32::pow(s, 3));


}