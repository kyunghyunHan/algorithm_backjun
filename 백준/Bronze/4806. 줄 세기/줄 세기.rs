use std::io::{self, BufRead};

fn count_lines(input_text: &str) -> usize {
    let lines: Vec<&str> = input_text.lines().collect();
    lines.len()
}

fn main() {
    let mut input_text = String::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            input_text.push_str(&line);
            input_text.push('\n');
        }
    }

    let result = count_lines(&input_text);
    println!("{}", result);
}
