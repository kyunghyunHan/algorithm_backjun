
use std::{io::{BufReader,BufRead,BufWriter,Write,stdin,stdout}, vec, cmp::Reverse};
fn main(){
   let mut reader= BufReader::new(stdin().lock());
   let mut writer= BufWriter::new(stdout().lock());
   let mut input = String::new();
   reader.read_line(&mut input).unwrap();
   let mut n:i32= input.trim().parse().unwrap();

   input.clear();
   reader.read_line(&mut input).unwrap();
   let mut a: Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    a.sort();
   input.clear();
   reader.read_line(&mut input).unwrap();
   let mut b: Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
   b.sort_by_key(|x|Reverse(*x));
   let mut v= vec![0;n as usize];

   for i in 0..n{
       v [i as usize]=b[i as usize] * a[i as usize];
   }

   writeln!(writer,"{}",v.iter().sum::<i32>()).unwrap();
}

