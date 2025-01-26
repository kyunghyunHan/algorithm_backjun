use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        let mut dp = [0u64; 100_001];

        dp[1] = 1; //1
        dp[2] = 1; //2
        dp[3] = 2; //2 1
        dp[4] = 2; //2 2
        dp[5] = 1; //5
        dp[6] = 2; //5 1
        dp[7] = 1; //7
                   //8 = 7 +1
                   //2
        for i in 8..=n {
            dp[i] = vec![
                dp.get(i - 1).unwrap_or(&u64::MAX), // i-1 인덱스가 유효하지 않으면 MAX 값을 사용
                dp.get(i - 2).unwrap_or(&u64::MAX),
                dp.get(i - 5).unwrap_or(&u64::MAX),
                dp.get(i - 7).unwrap_or(&u64::MAX),
            ]
            .iter()
            .copied()
            .min()
            .unwrap_or(&u64::MAX)
                + 1;
        }

        writeln!(writer, "{}", dp[n]).unwrap();
    }
    writer.flush().unwrap();
}
