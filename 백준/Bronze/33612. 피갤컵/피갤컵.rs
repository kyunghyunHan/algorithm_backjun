use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse().unwrap();

        match n {
            1 => {
                writeln!(writer, "{}", "2024 8").unwrap();
            }
            2 => {
                writeln!(writer, "{}", "2025 3").unwrap();
            }
            3 => {
                writeln!(writer, "{}", "2025 10").unwrap();
            }
            4 => {
                writeln!(writer, "{}", "2026 5").unwrap();
            }
            5 => {
                writeln!(writer, "{}", "2026 12").unwrap();
            }
            _ => {}
        }
    }
    writer.flush().unwrap();
}
