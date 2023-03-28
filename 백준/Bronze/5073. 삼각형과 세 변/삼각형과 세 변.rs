use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let nums: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];
        if a == 0 && b == 0 && c == 0 {
            break;
        }
        if a == b && b == c {
            println!("Equilateral");
        } else if a >= b + c || b >= c + a || c >= a + b {
            println!("Invalid");
        } else if a == b || b == c || c == a {
            println!("Isosceles");
        } else {
            println!("Scalene");
        }
    }
}