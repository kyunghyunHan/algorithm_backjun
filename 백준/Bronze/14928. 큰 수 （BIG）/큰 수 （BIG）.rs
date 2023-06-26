use std::io::{self, BufRead};

const MOD: u32 = 20000303;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let s = input.trim();

    let mut result = 0;

    for digit in s.chars() {
        let digit_val = digit.to_digit(10).unwrap();
        result = ((result * 10) % MOD + digit_val) % MOD;
    }

    println!("{}", result);
}
