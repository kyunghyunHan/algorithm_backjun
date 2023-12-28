use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut s = String::new();
    reader.read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    let len = s.len();
    let mut cnt = 0;

    for i in 0..(len - 3) {
        if &s[i..(i + 4)] == "DKSH" {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
