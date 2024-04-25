use std::collections::HashMap;
use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut buffer = String::new();
    input.read_line(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let d: i32 = iter.next().unwrap().parse().unwrap();

    let mut v: Vec<String> = Vec::with_capacity(n);
    for _ in 0..n {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        v.push(buffer.trim().to_string());
    }

    let m: HashMap<char, (char, char)> = [
        ('d', ('q', 'b')),
        ('q', ('d', 'p')),
        ('b', ('p', 'd')),
        ('p', ('b', 'q')),
    ]
    .iter()
    .cloned()
    .collect();

    for i in 0..n {
        for j in 0..n {
            if d == 1 {
                print!("{}", m[&v[i].chars().nth(j).unwrap()].0);
            } else {
                print!("{}", m[&v[i].chars().nth(j).unwrap()].1);
            }
        }
        println!();
    }
}

fn main() {
    solve();
}
