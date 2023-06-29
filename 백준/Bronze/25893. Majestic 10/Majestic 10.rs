use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::collections::HashMap;
use std::sync::Arc;
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input: String = String::new();
  reader.read_line(&mut input).unwrap();
  let n: usize = input.trim().parse().expect("Invalid input");

  for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut nums:Vec<usize>= input.split_whitespace().map(|x|x.parse().unwrap() ).collect();
    let mut count= 0;
    for i in &nums{
      if *i>=10{
        count+=1;
      }
    }
    match count {
        0 => writeln!(writer,"{} {} {}\n{}",nums[0],nums[1],nums[2],"zilch").unwrap(),
        1 => writeln!(writer,"{} {} {}\n{}",nums[0],nums[1],nums[2],"double").unwrap(),
        2 => writeln!(writer,"{} {} {}\n{}",nums[0],nums[1],nums[2],"double-double").unwrap(),
        3 => writeln!(writer,"{} {} {}\n{}",nums[0],nums[1],nums[2],"triple-double").unwrap(),
        _ => unreachable!(),

    }
    writeln!(writer).unwrap();
  }
}