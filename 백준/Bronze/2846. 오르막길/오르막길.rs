use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    stdin.lock().read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();

    buffer.clear();
    stdin.lock().read_line(&mut buffer).unwrap();
    let mut arr: Vec<i32> = buffer
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut sum = 0;
    let mut result = 0;

    for i in 1..n {
        if arr[i] > arr[i - 1] {
            sum += arr[i] - arr[i - 1];
        } else {
            result = result.max(sum);
            sum = 0;
        }
    }

    result = result.max(sum);

    println!("{}", result);
}
