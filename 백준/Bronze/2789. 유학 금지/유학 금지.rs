use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read line");
    let banned_chars = "CAMBRIDGE";
    let result = input.trim().chars().filter(|c| !banned_chars.contains(*c)).collect::<String>();
    println!("{}", result);
}
