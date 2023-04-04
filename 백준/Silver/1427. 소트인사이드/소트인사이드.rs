use std::io::{stdin, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n = buffer.trim();

    let mut v: Vec<i32> = vec![];
    for i in n.chars() {
        let u = i.to_digit(10).unwrap() as i32; // 입력받은 문자를 숫자로 변환
        v.push(u);
    }
    v.sort();
    v.reverse(); // 벡터를 뒤집음

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for i in v {
        write!(writer, "{}", i).unwrap(); // 뒤집힌 벡터의 각 숫자를 출력
    }
}