use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut i = 1;

    
    while i * i <= n {
        i += 1;
    }
    writeln!(writer,"The largest square has side length {}.", (i - 1) ).unwrap();


}