use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut lines = reader.lines();

    if let Some(Ok(line)) = lines.next() {
        let line: i32 = line.parse().unwrap();
        let mut sarr: HashMap<String, i32> = HashMap::new();
        for i in 0..line {
            let s = lines
                .next()
                .unwrap()
                .unwrap()
                .trim()
                .chars()
                .collect::<Vec<char>>()
                .first()
                .unwrap()
                .to_string();

            if sarr.contains_key(&s) {
                *sarr.entry(s).or_insert(0) += 1;
            } else {
                sarr.insert(s, 1);
            }
        }
        let mut result = Vec::new();
        for i in sarr {
            if i.1 >= 5 {
                result.push(i.0);
            }
        }
        if !result.is_empty() {
            result.sort();
            for i in result {
                print!("{}", i);
            }
        } else {
            println!("{}", "PREDAJA");
        }
    }
}
