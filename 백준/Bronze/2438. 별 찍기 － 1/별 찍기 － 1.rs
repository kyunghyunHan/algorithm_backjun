//별찍기
use std::fmt::Write;
use std::io;
fn main() {
    let mut numbers_arry = String::new();

    io::stdin().read_line(&mut numbers_arry).unwrap();

    let numbers: Vec<&str> = numbers_arry.split_whitespace().collect();

    let number: i32 = numbers[0].parse::<i32>().unwrap();

    for i in 1..number + 1 {
        for j in 1..i + 1 {
            if j <= i {
                print!("*");
            }
        }
        print!("\n");
    }
}
