use std::io::{self, BufRead};

fn is_balanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        if ch == '(' {
            stack.push(ch);
        } else if ch == ')' {
            if stack.is_empty() {
                return false;
            } else {
                stack.pop();
            }
        }
    }
    stack.is_empty()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    for _ in 0..n {
        let s = lines.next().unwrap()?.trim().to_string();
        if is_balanced(&s) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
    Ok(())
}
