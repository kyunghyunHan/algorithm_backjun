use std::io::{self, BufRead};

fn main() {
    // 입력
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..n {
        let v: Vec<i32> = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // 문제 해결
        let mut v_sorted = v.clone();
        v_sorted.sort();
        if v_sorted[3] - v_sorted[1] >= 4 {
            println!("KIN");
        } else {
            let ssum: i32 = v_sorted[1..4].iter().sum();
            println!("{}", ssum);
        }
    }
}
