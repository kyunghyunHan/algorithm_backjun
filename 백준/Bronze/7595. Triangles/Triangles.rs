use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    loop {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        if n == 0 {
            break;
        }
        for i in 1..=n{
            writeln!(writer,"{}", "*".repeat(i as usize)).unwrap();
        }
    }
    writer.flush().unwrap();
}
