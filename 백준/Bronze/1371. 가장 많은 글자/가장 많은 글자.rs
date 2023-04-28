use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut alphabet = [0; 26];
    let mut max = 0;
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        for ch in s.chars() {
            if ch.is_ascii_lowercase() {
                alphabet[ch as usize - 'a' as usize] += 1;
            }
        }
    }
    for &count in &alphabet {
        max = std::cmp::max(max, count);
    }
    for (i, &count) in alphabet.iter().enumerate() {
        if count == max {
            print!("{}", (b'a' + i as u8) as char);
        }
    }
    println!();
}
