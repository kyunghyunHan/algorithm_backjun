use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();
    
    if let Some(Ok(line)) = input.next() {
        let s = line;
        let chars: Vec<char> = s.chars().collect();
        
        // 첫 문자 출력
        if let Some(&first) = chars.first() {
            write!(writer, "{}", first).unwrap();
        }
        
        // 이전 문자와 다를 때만 출력 (continue 사용)
        for i in 1..chars.len() {
            if chars[i] == chars[i-1] {
                continue;
            }
            write!(writer, "{}", chars[i]).unwrap();
        }
    }
    writer.flush().unwrap();
}