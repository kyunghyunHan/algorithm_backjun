use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();
    match input.next() {
        Some(Ok(line)) => {
            let n = line.parse::<usize>().unwrap();
            let m = input.next().unwrap().unwrap().trim().to_string();
            let k = input
                .next()
                .unwrap()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let mut num = Vec::new();
            for c in m.chars().take(n) {
                num.push(c.to_digit(10).unwrap() as i32); // 0 또는 1로 변환
            }

            for _ in 0..k {
                if let Some(top_num) = num.pop() {
                    if top_num != 0 {
                        writeln!(writer,"NO").unwrap();
                        return;
                    }
                } else {
                    break; // 스택이 비어 있으면 중지
                }
            }
            writeln!(writer,"YES").unwrap();

        }
        _ => {
            writeln!(writer, "{}", "ERR!").unwrap();
        }
    }
    writer.flush().unwrap();
}
