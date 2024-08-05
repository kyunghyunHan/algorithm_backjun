use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();
    let mut v = HashMap::new();
    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let cache: i32 = input.trim().parse().unwrap();
        v.insert(i + 1, cache);
    }
    input.clear();
    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut result = 0;
    for _ in 1..=n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s: i32 = input.trim().parse().unwrap();
        let num = v.get(&s).unwrap();
        result += num;
    }
    writeln!(writer, "{}", result).unwrap();
}
