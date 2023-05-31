use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
let v1: [char; 26]= ['A', 'B','C', 'D','E' ,'F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z' ];
let v2: [char; 26]= ['D','E' ,'F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z' ,'A', 'B','C'];
let mut reader= BufReader::new(stdin().lock());

let mut writer= BufWriter::new(stdout().lock());
let mut input = String::new();
reader.read_line(&mut input).unwrap();

let s= input.trim();

for i in s.chars(){
   for j in 0..v2.len(){
  if i==v2[j]{
   write!(writer,"{}",v1[j]).unwrap();
   }
   }
}
}