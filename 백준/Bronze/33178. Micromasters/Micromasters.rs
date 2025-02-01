use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n: u64 = line.parse().unwrap();
        let result = n / 10;
        writeln!(writer, "{}", result).unwrap();
    }
    writer.flush().unwrap();
}
