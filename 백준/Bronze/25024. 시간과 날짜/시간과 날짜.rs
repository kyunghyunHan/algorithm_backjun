use std::io::{self, BufRead};

const DAYS: [u32; 13] = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let t: u32 = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        let mut iter = buffer.trim().split_whitespace();
        let x: u32 = iter.next().unwrap().parse().unwrap();
        let y: u32 = iter.next().unwrap().parse().unwrap();

        if (0 <= x && x <= 23) && (0 <= y && y <= 59) {
            print!("Yes ");
        } else {
            print!("No ");
        }

        if (1 <= x && x <= 12) && (1 <= y && y <= DAYS[x as usize]) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
