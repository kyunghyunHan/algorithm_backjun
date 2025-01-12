use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    match input.next() {
        Some(Ok(line)) => {
            let n = line.trim().parse::<u32>().unwrap();

            //1개또는 3개 가져갈수있음

            let mut dp = [0u32; 1001];
            //상근이가 먼저 시작
            dp[1] = 0; //상근이가 짐
            dp[2] = 1; //창영이가 짐
            dp[3] = 0; //상근이가짐
            dp[4] = 1; //창영이가짐

            for i in 5..=n {
                dp[i as usize] = if i % 2 == 0 { 1 } else { 0 };
            }

            if dp[n as usize]==1{
                writeln!(writer, "{}", "SK").unwrap();

            }else{
                writeln!(writer, "{}", "CY").unwrap();

            }
         }

        _ => {}
    }
    writer.flush().unwrap();
}
