use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();
    
    // 입력 라인을 처리
    match input.next() {
        Some(Ok(line)) => {
            let mut words = line.trim().split_whitespace();
            
            // 첫 번째와 두 번째 단어를 매칭
            match (words.next(), words.next()) {
                (Some(f), Some(s)) => {
                    let num1 = u128::from_str_radix(f, 2).unwrap();
                    let num2 = u128::from_str_radix(s, 2).unwrap();
                    let sum = num1 + num2;

                    writeln!(writer, "{}", format!("{:b}", sum)).unwrap();
                }
                _ => {
                    writeln!(writer, "두 단어를 입력해야 합니다.").unwrap();
                }
            }
        }
        Some(Err(e)) => {
            writeln!(writer, "입력을 읽는 중 문제가 발생했습니다: {}", e).unwrap();
        }
        None => {
            writeln!(writer, "입력이 없습니다.").unwrap();
        }
    }

    writer.flush().unwrap();
}
