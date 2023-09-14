use std::io;
fn main() {
    let mut numbers_arry = String::new();

    io::stdin().read_line(&mut numbers_arry).unwrap();

    let numbers: Vec<&str> = numbers_arry.split_whitespace().collect();

    let number_first: i32 = numbers[0].parse::<i32>().unwrap();

    for i in 1..10 {
        println!("{} * {} = {}", number_first, i, number_first * i)
    }
}
