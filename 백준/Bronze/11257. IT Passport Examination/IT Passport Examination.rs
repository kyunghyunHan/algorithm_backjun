use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
  let mut writer= BufWriter::new(stdout().lock());
  let n= input.trim().parse().unwrap();

  for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut total= 0;
    let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
     let mut  result= "";
     if nums[1]+nums[2]+nums[3]>=55 && nums[1]>=11 &&nums[2]>=8 && nums[3]>=12{
      result= "PASS";
     }else {
      result= "FAIL";
     }
    
    writeln!(writer,"{} {} {}",nums[0],nums[1]+nums[2]+nums[3],result).unwrap();

    
  }
}   