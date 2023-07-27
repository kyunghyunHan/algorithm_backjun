use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let bx: i32 = iter.next().expect("Invalid input").parse().expect("Invalid number");
    let by: i32 = iter.next().expect("Invalid input").parse().expect("Invalid number");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let dx: i32 = iter.next().expect("Invalid input").parse().expect("Invalid number");
    let dy: i32 = iter.next().expect("Invalid input").parse().expect("Invalid number");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let jx: i32 = iter.next().expect("Invalid input").parse().expect("Invalid number");
    let jy: i32 = iter.next().expect("Invalid input").parse().expect("Invalid number");

    let b = max((jx - bx).abs(), (jy - by).abs());
    let d = (jx - dx).abs() + (jy - dy).abs();

    if b == d {
        println!("tie");
    } else if b < d {
        println!("bessie");
    } else {
        println!("daisy");
    }
}

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}
