use core::num;
use std::{io::{BufRead,BufReader,Write,BufWriter,stdin,stdout}, result, iter::Sum};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let mut nums=input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());

    let a= nums.next().unwrap(); 
    let b= nums.next().unwrap(); 
    let c= nums.next().unwrap(); 
    let d= nums.next().unwrap(); 
    let e= nums.next().unwrap(); 
  
   let result_v=[a,b,c,d,e];
  let mut result= 0;
    for i in 0..5{
        result+=result_v[i]*result_v[i];
    }
   writeln!(writer,"{}",result%10).unwrap();
}


