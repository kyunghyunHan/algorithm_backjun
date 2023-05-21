use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(Ok(line)) = lines.next() {
        if line == "***" {
            break;
        }

        let reversed_sentence: String = line.chars().rev().collect();

        println!("{}", reversed_sentence);
    }
}
