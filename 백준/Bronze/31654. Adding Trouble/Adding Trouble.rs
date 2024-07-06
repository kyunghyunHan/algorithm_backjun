use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let tokens: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|token| token.parse().expect("Invalid input"))
        .collect();
    let a = tokens[0];
    let b = tokens[1];
    let c = tokens[2];
    if a + b == c {
        writeln!(writer,"correct!").unwrap();
    } else {
        writeln!(writer,"wrong!").unwrap();
    }
    writer.flush().unwrap();
}
