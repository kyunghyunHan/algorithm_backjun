use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut i = 1;
    let mut is_find = false;
    for line in stdin.lock().lines().take(5) {
        let str = line.unwrap();
        if str.contains("FBI") {
            print!("{} ", i);
            is_find = true;
        }
        i += 1;
    }
    if !is_find {
        println!("HE GOT AWAY!");
    }
}
