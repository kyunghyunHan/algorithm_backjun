use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};

fn main(){
let mut reader= BufReader::new(stdin().lock());

let mut input = String::new();
reader.read_line(&mut input).unwrap();

let n =input.trim().parse().unwrap();
let mut dp:[u64;100]= [0;100];
let mut writer= BufWriter::new(stdout().lock());
dp[0]= 1;
dp[1]= 1;
dp[2]= 1;
dp[3]= 2;
dp[4]= 2;

for i in 5..100 {
    dp[i] = dp[i - 1] + dp[i - 5];
}

for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut nums:u64= input.trim().parse().unwrap();
    writeln!(writer,"{}",dp[nums as usize-1]);
}
}

