use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input");
    let n: i32 = line.trim().parse().expect("Invalid input");

    for _ in 0..n {
        line.clear();
        reader.read_line(&mut line).expect("Failed to read input");
        let mut iter = line.trim().split_whitespace();
        let a: f64 = iter.next().unwrap().parse().expect("Invalid input");
        let b: f64 = iter.next().unwrap().parse().expect("Invalid input");

        let height = (2.0 * a) / b;
        println!("The height of the triangle is {:.2} units", height);
    }
}
