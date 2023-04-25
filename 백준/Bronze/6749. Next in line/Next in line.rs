use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());


  let mut writer= BufWriter::new(stdout().lock());
  let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
    let a= nums.next().unwrap();
   
    let mut input2= String::new();
    reader.read_line(&mut input2).unwrap();
    let mut nums2= input2.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
    let b= nums2.next().unwrap();
   
  
  writeln!(writer,"{}",b+(b-a)).unwrap();
}