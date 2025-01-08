use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::result;
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    let nm = match input.next() {
        Some(Ok(line)) => line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>(),
        _ => {
            panic!("Not n")
        }
    };
    let mut result = HashMap::new();
    for _ in 0..nm[0] {
        let _ = match input.next() {
            Some(Ok(line)) => line.trim().parse::<i32>().unwrap(),
            _ => {
                panic!("Not n")
            }
        };
        let study_info = match input.next() {
            Some(Ok(line)) => line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
            _ => {
                panic!("Not n")
            }
        };

        for student in study_info {
            *result.entry(student.to_string()).or_insert(0) += 1;
        }
    }
    let result = result.values().filter(|&&x| x >= nm[1]).count();

    writeln!(writer, "{}", result).unwrap();
    writer.flush().unwrap();
}
