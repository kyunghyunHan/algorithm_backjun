use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let  reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    match input.next() {
        Some(Ok(line)) => {
            let n = line.parse::<usize>().unwrap();

            let mut dp = [0u64;46];

            dp[1]= 1;
            dp[2] =1;
            dp[3]= 2;
      

            for i in 4..=n{
                dp[i] = dp[i -1 ] + dp[i-2];
            }

            writeln!(writer,"{} {}" ,dp[n-1], dp[n]).unwrap();
             
           /* 
           
         1  B 0 1
         2  BA 1 1 
         3   BAB 1 2
         4   BABBA 2 3
         5   BABBABAB 3 5
         
            
           
           
           
            */
        }
        _ => {
            panic!("eeorr");
        }
    }
    writer.flush().unwrap();
}
