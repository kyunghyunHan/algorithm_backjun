use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let mut iter = input.trim().split_whitespace();
    let p1: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let s1: i32 = iter.next().unwrap().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    iter = input.trim().split_whitespace();
    let s2: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let p2: i32 = iter.next().unwrap().parse().expect("Invalid input");

    if p1 + p2 > s1 + s2 {
        println!("Persepolis");
    } else if p1 + p2 < s1 + s2 {
        println!("Esteghlal");
    } else {
        if s1 > p2 {
            println!("Esteghlal");
        } else if s1 < p2 {
            println!("Persepolis");
        } else {
            println!("Penalty");
        }
    }
}
