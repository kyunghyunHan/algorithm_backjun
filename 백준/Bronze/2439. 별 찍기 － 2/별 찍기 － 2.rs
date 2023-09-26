//별찍기
use std::fmt::Write;
use std::io;
fn main() {
    let mut numbers_arry = String::new();

    io::stdin().read_line(&mut numbers_arry).unwrap();

    let numbers: Vec<&str> = numbers_arry.split_whitespace().collect();

    let number: i32 = numbers[0].parse::<i32>().unwrap();

    for i in 0..number {
        for j in 0..number {
            if j < number - i - 1 {
                print!(" ");
            } else {
                print!("*");
            }
        }
        print!("\n");
    }
}
