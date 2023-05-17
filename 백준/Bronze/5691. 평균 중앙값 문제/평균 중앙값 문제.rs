use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};
fn main(){
  let mut reader= BufReader::new(stdin().lock());

  let mut writer= BufWriter::new(stdout().lock());

  
  loop{
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    if nums[0] ==0 && nums[1]==0{
        break;
    }
        writeln!(writer,"{}",nums[0]*2-nums[1]).unwrap();

  }



}