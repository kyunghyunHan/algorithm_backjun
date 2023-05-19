use std::io::{self, BufRead};
use std::collections::HashSet;

fn is_pangram(sentence: &str) -> bool {
    let letters: HashSet<char> = sentence.chars().filter(|&c| c.is_ascii_lowercase()).collect();
    letters.len() == 26
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.expect("Failed to read line"));

    loop {
        let line = lines.next().unwrap();
        if line == "*" {
            break;
        }

        if is_pangram(&line) {
            println!("Y");
        } else {
            println!("N");
        }
    }
}
