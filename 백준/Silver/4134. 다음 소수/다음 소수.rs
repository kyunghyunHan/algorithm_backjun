use std::io::{stdin, BufRead};

fn is_prime(x: i64) -> bool {
    if x == 0 || x == 1 {
        return false;
    }
    for i in 2..=(x as f64).sqrt() as i64 {
        if x % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let stdin = stdin();
    let mut reader = stdin.lock().lines().map(|line| line.unwrap());
    let t: i32 = reader.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let z: i64 = reader.next().unwrap().trim().parse().unwrap();
        let mut num = z;
        loop {
            if is_prime(num) {
                println!("{}", num);
                break;
            } else {
                num += 1;
            }
        }
    }
}