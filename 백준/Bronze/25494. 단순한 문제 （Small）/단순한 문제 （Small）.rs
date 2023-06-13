use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 입력
    let sz: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..sz {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        let c: i32 = iter.next().unwrap().parse().unwrap();

        println!("{}", a.min(b).min(c));
    }
}
