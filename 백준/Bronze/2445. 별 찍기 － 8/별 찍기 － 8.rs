use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: i32 = input.trim().parse().expect("Failed to parse input");

    for i in 1..=n {
        for _ in 0..i {
            print!("*");
        }
        for _ in 0..2 * n - 2 * i {
            print!(" ");
        }
        for _ in 0..i {
            print!("*");
        }
        println!("");
    }

    for i in (1..n).rev() {
        for _ in 0..i {
            print!("*");
        }
        for _ in 0..2 * n - 2 * i {
            print!(" ");
        }
        for _ in 0..i {
            print!("*");
        }
        println!("");
    }
}
