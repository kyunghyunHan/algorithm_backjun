use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    let n: usize = input.next().unwrap().unwrap().parse().unwrap();

    let mut dp = vec![0.0; n];
    let mut arr = vec![0.0; n];

    // 입력값 읽기
    for i in 0..n {
        arr[i] = input.next().unwrap().unwrap().parse::<f64>().unwrap();
    }

    // DP 초기화
    dp[0] = arr[0];
    let mut max_product = dp[0];

    // DP 점화식 계산
    for i in 1..n {
        dp[i] = f64::max(dp[i - 1] * arr[i], arr[i]);
        max_product = f64::max(max_product, dp[i]);
    }

    // 결과 출력
    writeln!(writer, "{:.3}", max_product).unwrap();
}
