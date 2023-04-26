use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut numbers = input.split_whitespace().map(|x| x.trim().parse().expect("Invalid input"));
    let a:isize = numbers.next().expect("Invalid input");
    let b = numbers.next().expect("Invalid input");
    println!("{}", std::cmp::min(a / 2, b / 2));
}
