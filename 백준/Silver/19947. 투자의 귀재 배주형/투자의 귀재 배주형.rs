use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let hy = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let h = hy[0];
        let y = hy[1];
        
        // DP 배열 초기화 (y + 1 크기로 설정)
        let mut dp = vec![0; y + 1];
        dp[0] = h;

        for i in 1..=y {
            dp[i] = dp[i - 1] * 105 / 100; // A 방식
            if i >= 3 {
                dp[i] = dp[i].max(dp[i - 3] * 120 / 100); // B 방식
            }
            if i >= 5 {
                dp[i] = dp[i].max(dp[i - 5] * 135 / 100); // C 방식
            }
        }

        writeln!(writer, "{}", dp[y]).unwrap();
    }
}
