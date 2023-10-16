use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v1: Vec<i32> = input
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let N = v1[0];
    let mut count = 1;
    let mut i = 1;

    while N > i {
        i += (6 * count);
        count += 1;
    }

    println!("{}", count);
}
//벌집
