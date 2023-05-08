use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    let mut buffer = String::new();

    reader.read_line(&mut buffer).unwrap();
    let s = buffer.trim();

    for (i, c) in s.chars().enumerate() {
        write!(writer, "{}", c).unwrap();
        if (i + 1) % 10 == 0 {
            writeln!(writer).unwrap();
        }
    }

    writer.flush().unwrap();
}
