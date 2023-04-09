use std::io::{self, BufRead};
use std::cmp::Ordering;

fn cmp(a: &String, b: &String) -> Ordering {
    // 1. 길이가 같다면, 사전순으로
    if a.len() == b.len() {
        return a.cmp(b);
    }
    // 2. 길이가 다르다면, 짧은 순으로
    else {
        return a.len().cmp(&b.len());
    }
}

fn main() {
    let stdin = io::stdin();
    let mut word: Vec<String> = vec![];

    // N 입력받기
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // word 배열 입력받기
    for _ in 0..n {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        word.push(input.trim().to_string());
    }

    // word 정렬하기
    word.sort_by(cmp);

    // 중복 제거 후 출력
    let mut prev_word = "";
    for w in word.iter() {
        if w != prev_word {
            println!("{}", w);
            prev_word = w;
        }
    }
}