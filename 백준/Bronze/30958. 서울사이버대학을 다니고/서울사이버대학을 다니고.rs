use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::collections::HashMap;
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer = BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let s:Vec<char>= input.trim().chars().collect();
    let mut counts: HashMap<char, u32> = HashMap::new();

    for c in s {
        if c.is_ascii_alphabetic() {
            *counts.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }
    let mut max_char = ' ';
    let mut max_count = 0;

    for (c, &count) in counts.iter() {
        if count > max_count {
            max_count = count;
            max_char = *c;
        }
    }
    writeln!(writer,"{}", max_count).unwrap();
    writer.flush().unwrap();

}    

