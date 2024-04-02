use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.is_empty() {
                continue; // Skip empty line
            }
            let n: usize = line.parse().unwrap();
            let mut total_candies: u128 = 0;
            for _ in 0..n {
                let candies: u128 = lines.next().unwrap().parse().unwrap();
                total_candies += candies;
            }

            if total_candies % n as u128 == 0 {
                println!("YES");
            } else {
                println!("NO");
            }
            break;
        }
    }
}
