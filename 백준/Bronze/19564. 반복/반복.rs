use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    
    reader.read_line(&mut input).unwrap();
    let s = input.trim();
    
    let count = s.chars().zip(s.chars().skip(1))
        .filter(|(prev, curr)| curr <= prev)
        .count() + 1;
    
    writeln!(writer, "{}", count).unwrap();
}