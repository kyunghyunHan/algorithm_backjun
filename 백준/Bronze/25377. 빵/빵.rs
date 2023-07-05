use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
  let mut writer= BufWriter::new(stdout().lock());

  let n= input.trim().parse().unwrap();
  /*
  1.KOI빵을사려면 빵이 오기전에 가게에 가서 기다리고 있거나 빵이 오는 순간에 정확히 맞추어야 한다
  1.N개의 가게
  2.현재 위치에서 가게까지 걸리는 시간 a
  3.현재 시점에서 빵이 들어올떄까지 시간 B
  
   */
  let mut  result= 1001;
  for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut nums:Vec<i64>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();


    if nums[1] >=nums[0]{
        if nums[1]<result{
            result= nums[1]
        }
    }
  }  
  if result ==1001{
    writeln!(writer,"{}",-1).unwrap();
  }else {
    writeln!(writer,"{}",result).unwrap();

  }
}   