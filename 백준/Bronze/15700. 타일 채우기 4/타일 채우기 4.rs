use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut numbers = input.split_whitespace();
    let x: i64 = numbers.next().expect("Invalid input").parse().expect("Invalid input");
    let y: i64 = numbers.next().expect("Invalid input").parse().expect("Invalid input");
    println!("{}", x * y / 2);
}
