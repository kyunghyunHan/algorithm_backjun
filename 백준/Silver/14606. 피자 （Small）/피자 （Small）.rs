use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    match input.next() {
        Some(Ok(line)) => {
            let n = line.parse::<usize>().unwrap();

            let mut dp = [0u64; 11];
            //피자탑의 높이 A
            //높이 c와높이 B면
            // A = CB

            dp[1] = 0;
            dp[2] = 1;
        
            for i in 3..=n {
             
                let b = i / 2 + (i % 2);
                let c = i / 2;
                dp[i] = b as u64*c as u64 +dp[b] +dp[c];
                
            }
            writeln!(writer, "{}", dp[n]).unwrap();
        }
        _ => {
            panic!("eeorr");
        }
    }
    writer.flush().unwrap();
}
