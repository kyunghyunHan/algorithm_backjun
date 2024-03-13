use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::collections::HashMap;
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer = BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let n = input.trim().split_whitespace().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    writeln!(writer,"{}",(n[0]-n[1]).abs().max((n[1]-n[2]).abs())-1).unwrap();
    writer.flush().unwrap();

}    

