use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    stdin.lock().read_line(&mut line).unwrap();
    let available_meals: Vec<u32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    line.clear();
    stdin.lock().read_line(&mut line).unwrap();
    let requested_meals: Vec<u32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut shortage = 0;

    for i in 0..3 {
        if available_meals[i] < requested_meals[i] {
            shortage += requested_meals[i] - available_meals[i];
        }
    }

    println!("{}", shortage);
}
