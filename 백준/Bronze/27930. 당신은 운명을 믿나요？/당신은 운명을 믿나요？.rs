use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let mut reader = stdin().lock();
    let mut writer = BufWriter::new(stdout());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    
    let s = input.trim().chars().collect::<Vec<char>>();

    let korea = ['K', 'O', 'R', 'E', 'A'];
    let yonsei = ['Y', 'O', 'N', 'S', 'E', 'I'];

    let mut ki = 0; // KOREA의 문자 인덱스
    let mut yi = 0; // YONSEI의 문자 인덱스

    for c in s {
        if ki < 5 && c == korea[ki] {
            ki += 1; // KOREA 문자를 찾으면 인덱스 증가
        }
        if yi < 6 && c == yonsei[yi] {
            yi += 1; // YONSEI 문자를 찾으면 인덱스 증가
        }

        // 하나의 학교를 찾으면 바로 출력하고 종료
        if ki == 5 {
            writeln!(writer, "KOREA").unwrap();
            return;
        }
        if yi == 6 {
            writeln!(writer, "YONSEI").unwrap();
            return;
        }
    }
}
