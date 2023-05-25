use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut values = input.trim().split_whitespace();

    let n: i32 = values.next().unwrap().parse().expect("Invalid input");
    let m: i32 = values.next().unwrap().parse().expect("Invalid input");

    println!("{}", (n - 1) + ((m - 1) * n));
}
