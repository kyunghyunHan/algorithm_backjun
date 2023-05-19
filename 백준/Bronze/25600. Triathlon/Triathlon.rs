use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut max_score = 0;

    for _ in 0..n {
        let scores: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let a = scores[0];
        let d = scores[1];
        let g = scores[2];

        let score = if a == (d + g) {
            a * (d + g) * 2
        } else {
            a * (d + g)
        };

        if score > max_score {
            max_score = score;
        }
    }

    println!("{}", max_score);
}
