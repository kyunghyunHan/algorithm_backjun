use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    match input.next() {
        Some(Ok(line)) => {
            let n = line.parse::<usize>().unwrap();
            for i in (1..=n ){
              
                let spaces = " ".repeat(n - i);
                if i == 1 {
                    // 첫 줄
                    writeln!(writer, "{}*", spaces).unwrap();
                } else if i == n {
                    // 마지막 줄
                    writeln!(writer, "{}", "*".repeat(2 * n - 1)).unwrap();
                } else {
                    // 중간 줄
                    let middle_spaces = " ".repeat(2 * i - 3);
                    writeln!(writer, "{}*{}*", spaces, middle_spaces).unwrap();
                }
                
            }
        }
        _ => {}
    }
    writer.flush().unwrap();
}
