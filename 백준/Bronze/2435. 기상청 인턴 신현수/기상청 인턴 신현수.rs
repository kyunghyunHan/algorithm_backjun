use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // 첫 번째 줄 입력
    let (n, k) = {
        let line = input.next().unwrap().unwrap();
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (nums[0], nums[1])
    };

    // 두 번째 줄 입력
    let temperatures = {
        let line = input.next().unwrap().unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    };

    // 첫 번째 K일의 합을 초기값으로 설정
    let mut current_sum: i64 = temperatures.iter().take(k).sum();
    let mut max_sum = current_sum;

    // 슬라이딩 윈도우 적용
    for i in k..n {
        current_sum += temperatures[i] - temperatures[i - k];
        max_sum = max_sum.max(current_sum);
    }

    // 결과 출력
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    writeln!(writer, "{}", max_sum).unwrap();
}
