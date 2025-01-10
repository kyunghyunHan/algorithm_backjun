use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input  = reader.lines();

    match input.next(){
        Some(Ok(line))=>{
           let n = line.trim().parse::<usize>().unwrap();
           let mut dp = [0;41];
           dp[1] = 1;
           dp[2] = 2;

           for i in 3..=n{
            dp[i] = dp[i-2] +dp[i-1];
           }

           writeln!(writer,"{}",dp[n]).unwrap();
        }
        _=>{
            panic!("Error")
        }
    }
}