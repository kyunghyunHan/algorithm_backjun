use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let n1 = numbers[0];
    let k1 = numbers[1];
    let n2 = numbers[2];
    let k2 = numbers[3];

    println!("{}", n1 * k1 + n2 * k2);
}
