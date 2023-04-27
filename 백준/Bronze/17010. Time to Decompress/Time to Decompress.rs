use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let c: char = iter.next().unwrap().chars().next().unwrap();

        for _ in 0..n {
            print!("{}", c);
        }
        println!();
    }
}
