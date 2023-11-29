use std::io::{BufReader,BufWriter,Write,BufRead,stdin,stdout};



fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut numbers = input.trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    let a = numbers.next().unwrap();
    let b = numbers.next().unwrap();
    let c = numbers.next().unwrap();
    let d = numbers.next().unwrap();

    let result = ((a + d) - (b + c)).abs();
    writeln!(writer,"{}", result).unwrap();
    writer.flush().unwrap();
}
