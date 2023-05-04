use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let v: i32 = lines.next().unwrap().trim().parse().unwrap();
    let vote: Vec<char> = lines.next().unwrap().trim().chars().collect();

    let count_a = vote.iter().filter(|&c| *c == 'A').count();
    let count_b = vote.iter().filter(|&c| *c == 'B').count();

    if count_a > count_b {
        println!("A");
    } else if count_a < count_b {
        println!("B");
    } else {
        println!("Tie");
    }
}
