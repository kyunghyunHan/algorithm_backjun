use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer = BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();
    writeln!(writer, "{}", char::from_u32(n + 44032 - 1).unwrap()).unwrap();
    writer.flush().unwrap();
}
