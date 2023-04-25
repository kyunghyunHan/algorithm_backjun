use std::{io::{stdin,stdout,BufRead,BufReader,BufWriter,Write}, usize};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut input= String::new();
  let mut writer= BufWriter::new(stdout().lock());
  reader.read_line(&mut input).unwrap();
  let mut a= input.trim().parse::<usize>().unwrap();


  let mut input2= String::new();
  reader.read_line(&mut input2).unwrap();
  let mut b= input2.trim().parse::<usize>().unwrap();


writeln!(writer,"{}",a+b).unwrap();


}