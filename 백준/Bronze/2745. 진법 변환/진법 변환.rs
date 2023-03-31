use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let binding = lines.next().unwrap().unwrap();
    let mut input = binding.split_whitespace();
    let n = input.next().unwrap().to_string();
    let b = input.next().unwrap().parse::<i32>().unwrap();

    let mut result = 0;
    let mut cnt = 0;

    for c in n.chars().rev() {
        let digit_value = if c.is_ascii_alphabetic() {
            c as i32 - 'A' as i32 + 10
        } else {
            c.to_digit(10).unwrap() as i32
        };
        result += digit_value * b.pow(cnt);
        cnt += 1;
    }

    println!("{}", result);
}