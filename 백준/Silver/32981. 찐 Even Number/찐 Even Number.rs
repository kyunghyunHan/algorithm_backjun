use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut reeader = reader.lines();

    match reeader.next() {
        Some(Ok(line)) => {
            let n = line.parse::<usize>().unwrap();
            const MOD: u64 = 1_000_000_007;
            let mut dp = vec![0u64; 10000001];  // stack이 아닌 heap에 할당
            dp[1] = 5;  // 첫 자리는 {2,4,6,8} 4개
            dp[2] = 4 * 5;  // 첫 자리 4개 * 두번째 자리 5개 {0,2,4,6,8}
            for i in 3..=10_000_000 {
                dp[i] = (dp[i-1] * 5) % MOD;
            }
        
            for i in 0..n {
                match reeader.next() {
                    Some(Ok(line)) => {
                        let n = line.parse::<usize>().unwrap();
                        // DP 초기화
                        writeln!(writer,"{}",dp[n]).unwrap();
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
