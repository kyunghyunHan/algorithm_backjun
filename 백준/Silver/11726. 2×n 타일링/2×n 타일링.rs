use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){

    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
   let n :usize= input.trim().parse().unwrap();
    let mut dp= [0;1001];
    dp[1]= 1;
    dp[2]= 2;
    for i in 3..=n{
        dp[i]= dp[i-1]+dp[i-2] ;
        dp[i]%= 10007;
    }
    writeln!(writer,"{}",dp[n]).unwrap();
}
