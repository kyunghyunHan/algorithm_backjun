use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 첫 번째 줄에서 문자열 A를 읽어옴
    let string_a = lines.next().unwrap().unwrap();

    // 두 번째 줄에서 문자 목록 B를 읽어옴
    let string_b = lines.next().unwrap().unwrap();

    // 문자열 A를 문자 목록 B의 대응되는 소문자로 변경하여 문자열 C 생성
    let string_c: String = string_a
        .chars()
        .map(|c| {
            if string_b.contains(c) {
                c.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect();

    // 문자열 C 출력
    println!("{}", string_c);
}
