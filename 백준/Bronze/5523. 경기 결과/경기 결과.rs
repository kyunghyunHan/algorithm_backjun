use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let n: usize = lines.next().unwrap().parse().unwrap();
    
    let mut a_cnt = 0;
    let mut b_cnt = 0;
    
    for _ in 0..n {
        let ab: Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let a = ab[0];
        let b = ab[1];
        
        if a > b {
            a_cnt += 1;
        } else if a < b {
            b_cnt += 1;
        }
    }
    
    println!("{} {}", a_cnt, b_cnt);
}
