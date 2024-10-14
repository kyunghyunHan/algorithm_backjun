use std::collections::VecDeque;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write}; //원형큐
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut wirter = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut input = input
        .trim()
        .split_ascii_whitespace()
        .flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let k = input.next().unwrap();
    let mut queue: VecDeque<usize> = (1..=n).collect();
    let mut result = Vec::new();

    while !queue.is_empty() {
        for _ in 0..k - 1 {
            if let Some(front) = queue.pop_front() {
                queue.push_back(front);
            }
        }

        if let Some(person) = queue.pop_front() {
            result.push(person);
        }
    }
    write!(wirter, "<").unwrap();
    for i in 0..result.len() {
        if i < result.len()-1 {
            write!(wirter, "{}, ", result[i]).unwrap();
        } else {
            write!(wirter, "{}", result[i]).unwrap();
        }
    }
    write!(wirter, ">").unwrap();
}
