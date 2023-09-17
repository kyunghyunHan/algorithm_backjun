use core::num;
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut numbers= [0;10];
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
   let a= input.trim().parse::<i32>().unwrap();
   input.clear();
   reader.read_line(&mut input).unwrap();
   let b= input.trim().parse::<i32>().unwrap();
   input.clear();
   reader.read_line(&mut input).unwrap();
   let c= input.trim().parse::<i32>().unwrap();
   let result = (a*b*c).to_string();
   for digit in result.chars() {
    let digit_num = digit.to_digit(10).unwrap() as usize;
    numbers[digit_num] += 1;
}

   for i in numbers{
    writeln!(writer,"{}",i).unwrap();
   }
   
   writer.flush().unwrap();
}