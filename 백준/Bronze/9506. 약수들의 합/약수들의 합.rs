use std::io::{self, BufRead};

fn solution(n: i32) -> Option<Vec<i32>> {
    let mut divisors = Vec::new();
    let mut sum = 0;

    for i in 1..(n as f32).sqrt() as i32 + 1 {
        if n % i == 0 {
            divisors.push(i);
            if i > 1 {
                divisors.push(n / i);
                sum += n / i;
            }
            sum += i;
        }
    }

    if (n as f32).sqrt() as i32 * (n as f32).sqrt() as i32 == n {
        divisors.push((n as f32).sqrt() as i32);
        sum += (n as f32).sqrt() as i32;
    }

    if sum == n {
        divisors.sort();
        Some(divisors)
    } else {
        None
    }
}

fn main() {
    let stdin = io::stdin();
    loop {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();

        let n: i32 = line.trim().parse().unwrap();
        if n == -1 {
            break;
        }

        match solution(n) {
            Some(divisors) => {
                println!("{} = {}", n, divisors.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(" + "));
            },
            None => {
                println!("{} is NOT perfect.", n);
            },
        }
    }
}