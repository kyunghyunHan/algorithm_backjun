use std::io::{stdin,stdout};
use std::str::FromStr;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n:u64 = u64::from_str(line.trim()).unwrap();
    let mut k = n;
    let mut cnt = 0;
    while k != 1 {
        if k % 2 == 0 {
            k /= 2;
        } else {
            k = k + 1;
        }
        cnt += 1;
    }
    println!("{}", cnt);
}
