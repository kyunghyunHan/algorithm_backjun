use std::io::{self, BufRead};

fn get_grade(score: i32) -> &'static str {
    match score {
        97..=100 => "A+",
        90..=96 => "A",
        87..=89 => "B+",
        80..=86 => "B",
        77..=79 => "C+",
        70..=76 => "C",
        67..=69 => "D+",
        60..=66 => "D",
        _ => "F",
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    
    let t: i32 = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut words = line.split_whitespace();
        let name = words.next().unwrap();
        let score: i32 = words.next().unwrap().parse().unwrap();
        
        println!("{} {}", name, get_grade(score));
    }
}
