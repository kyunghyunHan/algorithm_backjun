use std::io;

fn main() {
    let mut b = String::new();

    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line");

    let b: i32 = b.trim().parse().expect("Invalid input");

    let p = 5 * b - 400;

    if p < 100 {
        println!("{}", p);
        println!("1");
    } else if p > 100 {
        println!("{}", p);
        println!("-1");
    } else {
        println!("{}", p);
        println!("0");
    }
}
