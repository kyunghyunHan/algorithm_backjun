use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let mut n: i32 = lines.next().unwrap().trim().parse().unwrap();
    let mut k = 0;
    let mut time = 0;
    loop {
        k += 1;
        if n == 0 {
            break;
        }
        if n < k {
            k = 1;
        }
        n -= k;
        time += 1;
    }
    println!("{}", time);
}
