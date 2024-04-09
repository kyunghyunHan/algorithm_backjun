use std::io;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input");
    let num: i32 = num.trim().parse().expect("Invalid input");

    let mut divisor = String::new();
    io::stdin().read_line(&mut divisor).expect("Failed to read input");
    let divisor: i32 = divisor.trim().parse().expect("Invalid input");

    let length = num.to_string().len();
    let mut num = num - (num % 100);

    loop {
        if num % divisor == 0 {
            break;
        }
        num += 1;
    }

    let num_str = num.to_string();
    let result = &num_str[length - 2..];
    println!("{}", result);
}
