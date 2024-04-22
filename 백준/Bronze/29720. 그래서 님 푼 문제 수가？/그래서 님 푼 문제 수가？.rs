use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let y= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    writeln!(writer,"{} {}",i32::max(0, y[0]-y[1]* y[2]),i32::max(0, y[0]-y[1]*(y[2]-1))-1).unwrap();
    writer.flush().unwrap();
}
