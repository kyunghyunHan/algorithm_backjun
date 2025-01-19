use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    match input.next(){
        Some(Ok(line))=>{
             let n = line.parse::<i32>().unwrap();
             let mut dp = [0i64;1_000_001];
             dp[1]= 1;
             dp[2]= 1;
             for i in 2..=n.abs(){
                dp[i as usize] = (dp[i as usize -1] + dp[i as usize -2]) % 1_000_000_000;
             }
             let result = if n% 2 ==0 && n<0{
                -1
             }else if n==0 {
                0
             }else{
                1
             };
             writeln!(writer,"{}",result).unwrap();
             writeln!(writer,"{}",dp[n.abs() as usize]).unwrap();
        } 
        _=>{

        }
    }
    writer.flush().unwrap();
}