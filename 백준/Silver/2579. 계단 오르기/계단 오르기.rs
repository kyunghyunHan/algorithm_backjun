use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};
use std::cmp::max;
fn main(){
let mut dp= [0;301];
let mut arr= [0;301];

    let mut reader= BufReader::new(stdin().lock());

    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let mut n= input.trim().parse().unwrap();
    for i in 1..=n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut s= input.trim().parse().unwrap();
        arr[i]=s;
    }
    dp[1]= arr[1];
    dp[2]= arr[1]+arr[2];
    dp[3]= max(arr[1]+arr[3],arr[2]+arr[3]);
    for i in 4..=n{
        dp[i]= max(dp[i-2]+arr[i],dp[i-3]+arr[i-1]+arr[i]);
    }

    println!("{}",dp[n]);
}

