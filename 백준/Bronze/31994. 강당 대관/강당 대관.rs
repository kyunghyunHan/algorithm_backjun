use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    let mut dic = HashMap::new();

    for _ in 0..7 {
        input.clear();
        reader.read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let semina = parts.next().expect("Expected a string").to_string();
        let num: i32 = parts
            .next()
            .expect("Expected a number")
            .parse()
            .expect("Invalid number");
        dic.insert(semina, num);
    }
    let max_key = dic.iter()
    .max_by_key(|entry| entry.1)
    .map(|(key, _)| key)
    .expect("Dictionary is empty");
    writeln!(writer,"{}",max_key).unwrap();
    writer.flush().unwrap();
}
