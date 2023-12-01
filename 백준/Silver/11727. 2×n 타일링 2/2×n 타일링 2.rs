use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){

    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
   let n :usize= input.trim().parse().unwrap();
    let mut dp= [0;1001];
    dp[1]= 1;
    dp[2]= 3;
    dp[3]=5;
    //a1= 1
    //a2= 2
    //a3= a1*2+a2
    //a4 =a2*2+a3
    //an = (an-2*2) +an-1
    for i in 4..=n{
        dp[i]=( dp[i-2]*2)+dp[i-1] ;
        dp[i]%= 10007;
    }
    writeln!(writer,"{}",dp[n]).unwrap();
    writer.flush().unwrap();
}
