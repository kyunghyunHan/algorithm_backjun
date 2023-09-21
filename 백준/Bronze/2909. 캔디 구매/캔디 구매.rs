use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut parts = input.trim().split_whitespace();
    let c: i32 = parts.next().unwrap().parse().expect("Failed to parse input");
    let k: i32 = parts.next().unwrap().parse().expect("Failed to parse input");

    let num = 10i32.pow(k as u32);
    let ans = ((c as f64 / num as f64).round() as i32) * num;

    println!("{}", ans);
}
