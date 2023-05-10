use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        let n: usize = buffer.trim().parse().unwrap();
        let mut d: [bool; 101] = [false; 101];
        for i in 1..=n {
            for j in 1..=n/i {
                d[i*j] = !d[i*j];
            }
        }
        let count = d.iter().skip(1).filter(|&&b| b).count();
        println!("{}", count);
    }
}
