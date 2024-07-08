use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let c: i32 = input.trim().parse().unwrap();

    writeln!(writer,"{}",(a*b)+c).unwrap();
    writer.flush().unwrap();
}
