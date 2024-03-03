use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let  s:f64= input.trim().parse().unwrap();

    writeln!(writer,"{:.10}",100f64/s).unwrap();

    writeln!(writer,"{:.10}",100f64/(100f64-s)).unwrap();

    
}
