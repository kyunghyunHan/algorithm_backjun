use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t= input.trim().parse::<i32>().unwrap();
    let w = t << 1;
    let len = (w as f64).sqrt() as i32;
    writeln!(writer,"{}",len*4).unwrap();
}
