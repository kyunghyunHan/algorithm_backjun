use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();

    let mut b = 0;
    let mut s = 0;
    let mut a = 0;

    for c in input.chars() {
        match c {
            'B' => b += 1,
            'S' => s += 1,
            'A' => a += 1,
            _ => (),
        }
    }

    if b == s && s == a && b == a {
        println!("SCU");
    } else {
        let max_val = b.max(s).max(a);
        if b == max_val {
            print!("B");
        }
        if s == max_val {
            print!("S");
        }
        if a == max_val {
            print!("A");
        }
    }
}
