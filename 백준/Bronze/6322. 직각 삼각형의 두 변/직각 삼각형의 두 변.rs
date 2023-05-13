use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let mut i = 1;
    loop {
        let input = lines.next().unwrap();
        let mut values = input.split_whitespace().map(|s| s.parse::<f64>().unwrap());

        let a = values.next().unwrap();
        let b = values.next().unwrap();
        let c = values.next().unwrap();

        if a == 0.0 && b == 0.0 && c == 0.0 {
            break;
        }

        println!("Triangle #{}", i);
        i += 1;

        let mut side = ' ';
        let mut result = 0.0;

        if a == -1.0 {
            side = 'a';
            result = (c * c - b * b).sqrt();
        } else if b == -1.0 {
            side = 'b';
            result = (c * c - a * a).sqrt();
        } else {
            side = 'c';
            result = (a * a + b * b).sqrt();
        }

        if result.is_nan() || result <= 0.0 {
            println!("Impossible.\n");
            continue;
        }

        println!("{} = {:.3}", side, result);
        println!();
    }
}
