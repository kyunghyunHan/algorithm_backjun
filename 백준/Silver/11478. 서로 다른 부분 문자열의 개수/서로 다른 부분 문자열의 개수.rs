use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();

    let mut set = HashSet::new();
    for i in 0..s.len() {
        let mut tmp = String::new();
        for j in i..s.len() {
            tmp.push(s.chars().nth(j).unwrap());
            set.insert(tmp.clone());
        }
    }

    println!("{}", set.len());
}