use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 첫 줄 입력 처리
    let first_line = lines.next().unwrap().unwrap();
    let mut split = first_line.split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let m: usize = split.next().unwrap().parse().unwrap();

    let mut results = Vec::new();
    let mut unique_streaks = std::collections::HashSet::new();

    // 각 동아리원의 기록 처리
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[m]; // 마지막 요소는 이름
        let record = &parts[..m]; // 앞의 M개의 요소는 활동 기록

        // 최장 리버스-스트릭 계산
        let mut max_streak = 0;
        let mut current_streak = 0;

        for &day in record {
            if day == "." {
                current_streak += 1;
                max_streak = max_streak.max(current_streak);
            } else {
                current_streak = 0;
            }
        }

        // 결과 저장 및 고유 리버스-스트릭 값 추가
        results.push((max_streak, name.to_string()));
        unique_streaks.insert(max_streak);
    }

    // 결과 출력
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    writeln!(writer, "{}", unique_streaks.len()).unwrap();
    for (streak, name) in results {
        writeln!(writer, "{} {}", streak, name).unwrap();
    }
}
