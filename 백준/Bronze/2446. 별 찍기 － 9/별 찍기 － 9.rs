use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input");
    let n: i32 = n.trim().parse().expect("Invalid input");

    for i in 1..=n {
        for _ in 0..(i-1) {
            print!(" ");
        }
        for _ in 0..(2*n-i*2+1) {
            print!("*");
        }
        println!();
    }

    for i in (1..n).rev() {
        for _ in 0..(i-1) {
            print!(" ");
        }
        for _ in 0..(2*n-2*i+1) {
            print!("*");
        }
        println!();
    }
}
