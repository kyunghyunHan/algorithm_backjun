use std::cmp;
use std::io::{BufReader,BufRead,BufWriter,stdin,Write,stdout};

fn main(){

    let mut reader= BufReader::new(stdin().lock());

    let mut input= String::new();
let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();

    let mut dp=[0;1001];

    let n: i32= input.trim().parse().unwrap();
    dp[1]= 1;
    dp[2]= 2;
    dp[3]= 4;
  for j in 4..11{
    dp[j]= dp[j-1]+dp[j-2]+dp[j-3];
  }

  for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let n: i32= input.trim().parse().unwrap();
     writeln!(writer,"{}",dp[ n as usize] ).unwrap();
  }
}