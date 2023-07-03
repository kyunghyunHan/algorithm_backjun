use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());

  let mut input = String::new();
  reader.read_line(&mut input).unwrap();
  let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

  let a=nums[0];
  let b=nums[1];
  let c=nums[2];
  let d=nums[3];

  /*     
  masha:a*b
  petya:c*d
  
  car :M
  Petya :P
  == :E
  
   */

  let masha= a*b;
  let petya= c*d;
  if masha >petya{
    writeln!(writer,"{}","M").unwrap();
  }else if masha<petya{
    writeln!(writer,"{}","P").unwrap();

  }else if masha==petya{
    writeln!(writer,"{}","E").unwrap();

  }
}