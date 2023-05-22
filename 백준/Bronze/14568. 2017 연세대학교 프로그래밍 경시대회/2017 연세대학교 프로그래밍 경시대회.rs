use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let n: i32 = input.trim().parse().expect("Invalid input.");

    let mut s = 0;
    for i in (2..n-1).step_by(2) {
        s += (n - i - 2) / 2;
    }

    println!("{}", s);
}
