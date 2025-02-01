use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n: f64 = line.parse().unwrap();
        let result = n.floor() as u32;

        writeln!(writer, "{}", result).unwrap();
    }
}
