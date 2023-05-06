use core::num;
use std::{io::{BufWriter,BufReader,BufRead,Write,stdin,stdout}, result};

fn main(){

let mut reader= BufReader::new(stdin().lock());
let mut result:(usize,usize)= (0,0);
let mut writer= BufWriter::new(stdout().lock());

for i in 0..5{
   let mut buffer= String::new();
reader.read_line(&mut buffer).unwrap();

let mut nums= buffer.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
let a= nums.next().unwrap();
let b= nums.next().unwrap();
let c= nums.next().unwrap();
let d= nums.next().unwrap();

if result.1< a+b+c+d{
   result= (i+1,a+b+c+d)
}




}
writeln!(writer,"{} {}",result.0,result.1).unwrap();
   


}