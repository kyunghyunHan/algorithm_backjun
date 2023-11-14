use std::collections::HashMap;
use std::io::{self, BufRead};

fn mid_score(m:u32) {
    let scores: Vec<u32> = read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let len_score = scores.len();

    let mut score_dict: HashMap<String, u32> = HashMap::new();

    for _ in 0..m {
        let data: Vec<String> = read_line().split_whitespace().map(String::from).collect();
        // num: 수험번호, arr: 채점결과
        let num = &data[0];
        let arr = &data[1..];

        let entry = score_dict.entry(num.clone()).or_insert(0);

        for j in 0..len_score {
            // 정답(O)이면, 점수 더 해주기
            if arr[j] == "O" {
                *entry += scores[j];
            }
        }
    }

    // min_student: 수험 번호는 최대 100_000
    let mut min_student = 100001;
    // max_val: 가장 높은 점수
    let max_val = *score_dict.values().max().unwrap();

    for (k, v) in score_dict.iter() {
        if v == &max_val {
            let student_num = k.parse::<u32>().unwrap();
            min_student = min_student.min(student_num);
        }
    }

    println!("{} {}", min_student, max_val);
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let nm: Vec<u32> = read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = nm[0];
    let m = nm[1];

    mid_score(m);
}
