use std::io::{BufReader,BufWriter,Write,BufRead,stdin,stdout};
use std::cmp::min;
fn main(){
    let  mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
     //2원과 5원짜리로만 거스름돈을 달라고함
     //동전의 갯수가 최대로 거슬로 줘야함

     /*
     1 => -1
     2 => 1
     3 => -1
    4=> 2
    5=>1
    6=> 3
    7=> 2
    8 =>4
    9=>3 
    10 => 2
    11 => 4
    12 => 3
    13 => 5
     
      */
    match input.next(){
        Some(Ok(line))=>{
            let n = line.parse::<usize>().unwrap();
            let mut dp = [-1i64;100001];
            dp[2]= 1;
            dp[5]= 1;

            for i  in 3..=n{
                if dp[i-2] !=-1{
                    dp[i] = dp[i-2]+1;
                }
                if i > 5 &&dp[i-5] !=-1 {
                    dp[i] = min(dp[i-5]+1,dp[i])

                }
                   
            }
            writeln!(writer,"{}",dp[n]).unwrap();   
        }
        _=>{}


    }

    writer.flush().unwrap();
}