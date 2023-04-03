use std::io::{stdin, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();

    // 카운팅 배열 초기화
    let mut counting: [usize; 10001] = [0; 10001];

    // 카운팅 정렬 (입력)
    for _ in 0..n {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        let input: usize = buffer.trim().parse().unwrap();
        counting[input] += 1;
    }

    // 카운팅 정렬 (출력)
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for i in 0..10001 {
        for _ in 0..counting[i] {
            writeln!(writer, "{}", i).unwrap();
        }
    }
}