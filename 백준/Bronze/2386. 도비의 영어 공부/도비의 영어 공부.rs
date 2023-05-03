use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    loop {
        let mut cnt = 0;
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        let c = iter.next().unwrap().chars().next().unwrap().to_ascii_lowercase();
        if c == '#' {
            return;
        }
        let k = iter.collect::<Vec<&str>>().join(" ");
        for ch in k.chars() {
            if c.is_ascii_uppercase() {
                if ch == c || ch == c.to_ascii_lowercase() {
                    cnt += 1;
                }
            } else if c.is_ascii_lowercase() {
                if ch == c || ch == c.to_ascii_uppercase() {
                    cnt += 1;
                }
            }
        }
        println!("{} {}", c.to_ascii_lowercase(), cnt);
    }
}
