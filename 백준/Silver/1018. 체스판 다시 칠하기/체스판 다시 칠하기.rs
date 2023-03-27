use std::io::{self, BufRead};

const WB: [&str; 8] = [
    "WBWBWBWB",
    "BWBWBWBW",
    "WBWBWBWB",
    "BWBWBWBW",
    "WBWBWBWB",
    "BWBWBWBW",
    "WBWBWBWB",
    "BWBWBWBW"
];

const BW: [&str; 8] = [
    "BWBWBWBW",
    "WBWBWBWB",
    "BWBWBWBW",
    "WBWBWBWB",
    "BWBWBWBW",
    "WBWBWBWB",
    "BWBWBWBW",
    "WBWBWBWB"
];

fn wb_cnt(x: usize, y: usize, board: &[String]) -> usize {
    let mut cnt = 0;
    for i in 0..8 {
        for j in 0..8 {
            if board[x+i].chars().nth(y+j) != WB[i].chars().nth(j) {
                cnt += 1;
            }
        }
    }
    cnt
}

fn bw_cnt(x: usize, y: usize, board: &[String]) -> usize {
    let mut cnt = 0;
    for i in 0..8 {
        for j in 0..8 {
            if board[x+i].chars().nth(y+j) != BW[i].chars().nth(j) {
                cnt += 1;
            }
        }
    }
    cnt
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let p1: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut board: Vec<String> = Vec::new();
    for _ in 0..p1[0] {
        board.push(lines.next().unwrap());
    }

    let mut min_val = usize::MAX;
    for i in 0..(p1[0]-8+1) {
        for j in 0..(p1[1]-8+1) {
            let tmp = std::cmp::min(wb_cnt(i, j, &board), bw_cnt(i, j, &board));
            if tmp < min_val {
                min_val = tmp;
            }
        }
    }

    println!("{}", min_val);
}