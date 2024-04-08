use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut split_iter = input.trim().split_whitespace();
    let a: i64 = split_iter.next().unwrap().parse().expect("Invalid input");
    let b: i64 = split_iter.next().unwrap().parse().expect("Invalid input");

    let mut result: i64 = 1;

    for i in a..=b {
        let mut sum = 0;
        for j in 1..=i {
            sum += j;
        }
        result = (result * sum) % 14579;
    }

    println!("{}", result);
}
