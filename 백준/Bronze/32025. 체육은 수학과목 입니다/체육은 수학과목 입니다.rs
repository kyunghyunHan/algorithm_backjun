use std::{
    cmp::min,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let a = input.trim().parse::<i32>().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let b = input.trim().parse::<i32>().unwrap();

    writeln!(writer, "{}", min(a, b) * 100 / 2).unwrap();
    writer.flush().unwrap();
}
