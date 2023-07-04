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
    let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let mut result= 0;
    for i in 0..nums[0]{
      if i==0{
        result+=nums[1];

      }else{
          result+=(nums[1]-2);

      }
    }
    writeln!(writer,"{} {}\n{}",nums[0],nums[1],result).unwrap();
  }
}