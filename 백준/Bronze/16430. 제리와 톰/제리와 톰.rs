use std::io::{BufWriter, BufRead, BufReader, Write, stdin, stdout};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    // Read n from input
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut num = input.trim().split_whitespace().map(|x| x.parse::<isize>().unwrap());
    let a= num.next().unwrap();
    let b= num.next().unwrap();

    writeln!(writer,"{} {}",b-a,b).unwrap();
}