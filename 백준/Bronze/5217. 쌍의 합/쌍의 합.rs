use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let test_case: i32 = input.trim().parse().unwrap();

    for _ in 0..test_case {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let num: i32 = input.trim().parse().unwrap();
        print!("Pairs for {}: ", num);

        let mut cnt = 0;
        for i in 1..=12 {
            for j in 1..=12 {
                if i + j == num && i < j {
                    if cnt == 0 {
                        cnt += 1;
                        print!("{} {}", i, j);
                    } else {
                        print!(", {} {}", i, j);
                    }
                }
            }
        }
        println!();
    }
}
