use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input: String = String::new();
  reader.read_line(&mut input).unwrap();
  let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

  
  /*
  같은숫자면 다른숫자만 탈락

  
   */
  //모두 같은 경우
  if nums[0] ==nums[1]&&nums[0]==nums[2]{
    writeln!(writer,"{}","*").unwrap();
    //승자 A
  }else if nums[0]!=nums[1]&&nums[0]!=nums[2]{
    writeln!(writer,"{}","A").unwrap();
//승자 B
  }else if nums[1]!=nums[0]&&nums[1]!=nums[2]{
    writeln!(writer,"{}","B").unwrap();
  }else if nums[2]!=nums[0]&&nums[2]!=nums[1]{
    writeln!(writer,"{}","C").unwrap();
  }
}