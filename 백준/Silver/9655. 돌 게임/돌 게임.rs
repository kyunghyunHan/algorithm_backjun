use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){

  let mut reader= BufReader::new(stdin().lock());
  let mut input = String::new();

  reader.read_line(&mut input).unwrap();

  let n:usize= input.trim().parse().unwrap();//돌의 갯수
let mut writer= BufWriter::new(stdout().lock());
//   let mut dp= [0;1000];

//   dp[0]= 0;
//   dp[1]= 0;
//   dp[2]= 0;
//   for i in 3..=n{
//     dp[i]= usize::min(dp[i-1]+1,dp[i-3]+1 );
//   }

//   //게임과정 횟의 홀짝에 따라 승패결정

//   if dp[n]%2==1{
//     writeln!(writer,"{}","SK").unwrap();
//   }else{
//     writeln!(writer,"{}","CY").unwrap();
//   }

if n%2==1{
  writeln!(writer,"{}","SK").unwrap();
}else{
  writeln!(writer,"{}","CY").unwrap();
}
}