use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut numbers = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect::<Vec<f64>>();

    let num1 = (numbers[0] * numbers[1]) / numbers[2];
    let num2 = (numbers[0] / numbers[1]) * numbers[2];

    println!("{}", num1.max(num2) as i64);
}
