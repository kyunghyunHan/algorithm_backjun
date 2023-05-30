use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
    let mut input =String::new();
    reader.read_line(&mut input).unwrap();
    let n:u128= input.trim().parse().unwrap();
   let s=  fibo(n);

   writeln!(writer,"{}",s).unwrap();
   
}
fn fibo(n: u128) -> u128 {
  let mut dp: Vec<u128> = vec![0; 200];  // 하위 문제 답을 저장할 메모이제이션 벡터
  dp[0] = 0;
  dp[1] = 1;
  for i in 2..=n {    // 2부터 시작해서 n까지 반복
      dp[i as usize] = dp[(i - 1) as usize] + dp[(i - 2) as usize];
  }
  dp[n as usize]
}