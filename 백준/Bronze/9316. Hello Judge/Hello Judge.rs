use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};
use std::cmp;
fn main(){
  let mut reader= BufReader::new(stdin().lock());


  let mut writer= BufWriter::new(stdout().lock());
  let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
    let a= nums.next().unwrap();
   

   for i in 1..=a{
      writeln!(writer,"Hello World, Judge {}!",i).unwrap();
   }
 
  

}