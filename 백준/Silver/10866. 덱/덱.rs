use std::collections::VecDeque;
use std::io;

fn main() {
    let mut dq: VecDeque<i32> = VecDeque::new();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: i32 = input.trim().parse().expect("Invalid input");

    for _ in 0..n {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read command");
        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts[0] {
            "push_front" => {
                let x: i32 = parts[1].trim().parse().expect("Invalid input");
                dq.push_front(x);
            }
            "push_back" => {
                let x: i32 = parts[1].trim().parse().expect("Invalid input");
                dq.push_back(x);
            }
            "pop_front" => {
                if let Some(x) = dq.pop_front() {
                    println!("{}", x);
                } else {
                    println!("-1");
                }
            }
            "pop_back" => {
                if let Some(x) = dq.pop_back() {
                    println!("{}", x);
                } else {
                    println!("-1");
                }
            }
            "size" => {
                println!("{}", dq.len());
            }
            "empty" => {
                println!("{}", dq.is_empty() as i32);
            }
            "front" => {
                if let Some(&x) = dq.front() {
                    println!("{}", x);
                } else {
                    println!("-1");
                }
            }
            "back" => {
                if let Some(&x) = dq.back() {
                    println!("{}", x);
                } else {
                    println!("-1");
                }
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}
