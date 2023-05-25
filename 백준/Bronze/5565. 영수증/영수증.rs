use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let totalPrice: i32 = input.trim().parse().expect("Invalid input");

    let mut sum = 0;

    for _ in 0..9 {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input");
        sum += num;
    }

    println!("{}", totalPrice - sum);
}
