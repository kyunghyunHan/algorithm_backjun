use std::fmt::Write;
use std::io;
fn main() {
    let mut numbers_arry = String::new();

    io::stdin().read_line(&mut numbers_arry).unwrap();

    let numbers: Vec<&str> = numbers_arry.split_whitespace().collect();

    let number: i32 = numbers[0].parse::<i32>().unwrap();
    let mut output = String::new();
    for i in 1..number + 1 {
        writeln!(output, "{}", i).unwrap();
    }
    print!("{}", output);
}