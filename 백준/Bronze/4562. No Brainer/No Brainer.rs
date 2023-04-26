use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
use std::cmp::max;
use std::result;
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());

  let mut input= String::new();

  reader.read_line(&mut input).unwrap();

  let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());

  let a= nums.next().unwrap();
  

  for i in 0..a{

input.clear();

reader.read_line(&mut input).unwrap();
let mut nums2= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());

let  x= nums2.next().unwrap();
 let  y= nums2.next().unwrap();

 if x<y{
   writeln!(writer,"{}","NO BRAINS").unwrap();
 }else{
  writeln!(writer,"{}","MMM BRAINS").unwrap();
 }


  }



  



}