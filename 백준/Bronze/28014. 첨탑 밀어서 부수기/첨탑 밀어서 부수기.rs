use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut count= 1;
    //첨탑을 밀려면 
    let mut nums: Vec<i32>= input.trim().split_ascii_whitespace().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    for i in 1..n{
      //전 인덱스 비교
         if nums[i as usize]>=nums[i as usize-1]{
          count+=1;
         }
    }
    writeln!(writer,"{}",count).unwrap();
  writer.flush().unwrap();
}