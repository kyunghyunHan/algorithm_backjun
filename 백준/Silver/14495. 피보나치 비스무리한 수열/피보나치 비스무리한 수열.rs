use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer =BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    match input.next(){
        Some(Ok(line))=>{
           let n = line.parse::<usize>().unwrap();
            let mut dp= [0u64;117];

            dp[1] = 1;
            dp[2]= 1;
            dp[3]= 1;
            dp[4]= 2;
            dp[5]= 3;

            for i in 6..=n{
                dp[i] = dp[i-1]+dp[i-3];
            }

            writeln!(writer,"{}",dp[n]).unwrap();

        }
        _=>{

        }
    }
    writer.flush().unwrap();
}