use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let h: usize = input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m : usize = input.trim().parse().unwrap();

    writeln!(writer,"{}",h*60+m).unwrap();
    writer.flush().unwrap();
}
