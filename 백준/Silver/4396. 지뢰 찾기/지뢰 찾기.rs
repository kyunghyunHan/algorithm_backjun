use std::io::{self, BufRead};

fn dfs(x: usize, y: usize, map: &[Vec<char>], n: usize) -> usize {
    let dx: [i32; 8] = [0, 0, -1, 1, -1, -1, 1, 1];
    let dy: [i32; 8] = [-1, 1, 0, 0, -1, 1, -1, 1];
    let mut cnt = 0;
    for i in 0..8 {
        let nx = x as i32 + dx[i];
        let ny = y as i32 + dy[i];
        if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
            if map[nx as usize][ny as usize] == '*' {
                cnt += 1;
            }
        }
    }
    cnt
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut map = vec![vec!['.'; n]; n];
    let mut inp = vec![vec!['.'; n]; n];
    let mut out = vec![vec!['.'; n]; n];
    let mut flag = false;
    for i in 0..n {
        let line: Vec<char> = lines.next().unwrap().chars().collect();
        map[i] = line;
    }
    for i in 0..n {
        let line: Vec<char> = lines.next().unwrap().chars().collect();
        inp[i] = line;
    }
    for i in 0..n {
        for k in 0..n {
            if inp[i][k] == 'x' {
                let cnt = dfs(i, k, &map, n);
                out[i][k] = (cnt as u8 + b'0') as char;
                if map[i][k] == '*' {
                    flag = true;
                }
            } else if inp[i][k] == '.' {
                out[i][k] = '.';
            }
        }
    }
    if flag {
        for i in 0..n {
            for k in 0..n {
                if map[i][k] == '*' {
                    out[i][k] = '*';
                }
            }
        }
    }
    for i in 0..n {
        for k in 0..n {
            print!("{}", out[i][k]);
        }
        println!("");
    }
}
