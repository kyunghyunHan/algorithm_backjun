use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut i = 1;
    while let Ok(n) = lines.next().unwrap().parse::<i32>() {
        if n == 0 {
            break;
        }

        let n1 = 3 * n;
        let n2 = (n1 + 1) / 2;
        let n3 = 3 * n2;
        let n4 = n3 / 9;

        println!("{}. {} {}", i, if n1 % 2 == 0 { "even" } else { "odd" }, n4);
        i += 1;
    }
}
