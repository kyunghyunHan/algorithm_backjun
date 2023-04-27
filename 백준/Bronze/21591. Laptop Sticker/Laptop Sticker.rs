use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let w1: i32 = iter.next().unwrap().parse().unwrap();
    let h1: i32 = iter.next().unwrap().parse().unwrap();
    let w2: i32 = iter.next().unwrap().parse().unwrap();
    let h2: i32 = iter.next().unwrap().parse().unwrap();

    if w1 - w2 >= 2 && h1 - h2 >= 2 {
        println!("1");
    } else {
        println!("0");
    }
}
