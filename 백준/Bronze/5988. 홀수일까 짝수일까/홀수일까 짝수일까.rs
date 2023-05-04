use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let n = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..n {
        let k = lines.next().unwrap().trim().to_string();
        let num = k.chars().last().unwrap().to_digit(10).unwrap() as i32;
        if num % 2 == 0 {
            println!("even");
        } else {
            println!("odd");
        }
    }
}
