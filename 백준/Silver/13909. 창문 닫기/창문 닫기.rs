use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut reader = stdin.lock();

    // n값 읽어들이기
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<i32>().unwrap();

    let mut i = 0;
    while i * i <= n {
        i += 1;
    }
    println!("{}", i - 1);
}