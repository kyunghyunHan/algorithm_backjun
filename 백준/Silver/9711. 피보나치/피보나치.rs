use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    if let Some(Ok(line)) = input.next() {
        let t = line.parse::<usize>().unwrap();
        let mut dp = vec![0u64; 10001];
        dp[1] = 1;
        dp[2] = 1;

        for i in 1..=t {
            if let Some(Ok(line)) = input.next() {
                let nums: Vec<u64> = line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
                let p = nums[0] as usize;
                let q = nums[1];

                // 각 테스트 케이스마다 피보나치 수열을 새로 계산
                dp[1] = 1;
                dp[2] = 1;
                for j in 3..=p {
                    dp[j] = (dp[j-1] + dp[j-2]) % q;
                }
                
                writeln!(writer, "Case #{}: {}", i, dp[p] % q).unwrap();
            }
        }
    }
}