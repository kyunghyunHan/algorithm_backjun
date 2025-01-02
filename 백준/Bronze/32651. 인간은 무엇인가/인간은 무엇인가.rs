use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut line = reader.lines();

    if let Some(Ok(line)) = line.next() {
        let n = line.parse::<i32>().unwrap();
        if n % 2024 == 0 && n <= 100000 {
            writeln!(writer, "Yes").unwrap();
        } else {
            writeln!(writer, "No").unwrap();
        }
    }
    writer.flush().unwrap();
}
