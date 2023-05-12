use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_line(&mut buf).unwrap();
    let t: usize = buf.trim().parse().unwrap();
    for _ in 0..t {
        buf.clear();
        stdin.lock().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();
        buf.clear();
        stdin.lock().read_line(&mut buf).unwrap();
        let mut v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
        v.sort();
        let mut res = 0;
        for j in 1..n {
            res += v[j] - v[j - 1];
        }
        println!("{}", res * 2);
    }
}
