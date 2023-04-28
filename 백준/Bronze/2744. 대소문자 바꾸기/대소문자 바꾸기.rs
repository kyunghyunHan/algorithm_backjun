use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let s = stdin.lock().lines().next().unwrap().unwrap();
    let mut sb = String::new();
    for c in s.chars() {
        if c.is_uppercase() {
            sb.push(c.to_lowercase().next().unwrap());
        } else {
            sb.push(c.to_uppercase().next().unwrap());
        }
    }
    println!("{}", sb);
}
