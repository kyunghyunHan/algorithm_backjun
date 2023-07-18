use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let ts: usize = lines.next().unwrap().parse().unwrap();
    
    for i in 1..=ts {
        let s: String = lines.next().unwrap();
        println!("String #{}", i);
        
        let transformed: String = s.chars()
            .map(|c| ((c as u8 - b'A' + 1) % 26 + b'A') as char)
            .collect();
        
        println!("{}", transformed);
        println!();
    }
}
