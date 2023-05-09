use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let params: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let r = params[0];
    let c = params[1];
    let zr = params[2];
    let zc = params[3];

    let mut scanner = vec![vec![' '; c + 1]; r + 1];

    for i in 1..=r {
        let row = lines.next().unwrap().trim().chars().collect::<Vec<char>>();
        for j in 1..=c {
            scanner[i][j] = row[j - 1];
        }
    }

    for i in 1..=r {
        for _ in 0..zr {
            for j in 1..=c {
                for _ in 0..zc {
                    print!("{}", scanner[i][j]);
                }
            }
            println!();
        }
    }
}
