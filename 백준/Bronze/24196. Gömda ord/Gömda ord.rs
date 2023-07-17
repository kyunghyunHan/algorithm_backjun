use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut encrt = String::new();
    reader.read_line(&mut encrt).expect("Failed to read input");

    let mut ans = String::new();
    let mut idx = 0;
    while idx < encrt.len() - 1 {
        ans.push(encrt.chars().nth(idx).unwrap());
        idx += encrt.chars().nth(idx).unwrap() as usize - 'A' as usize + 1;
    }

    ans.push(encrt.chars().last().unwrap());
    println!("{}", ans);
}
