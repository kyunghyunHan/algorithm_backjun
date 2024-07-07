use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let x: i32 = input.trim().parse().unwrap();

    if x % 7 == 2 {
        writeln!(writer, "1").unwrap();
    } else {
        writeln!(writer, "0").unwrap();
    }

    writer.flush().unwrap();
}
