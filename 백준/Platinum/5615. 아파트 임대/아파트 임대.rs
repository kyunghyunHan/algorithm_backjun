use std::io::{self, BufRead};

fn power(x: u64, y: u64, p: u64) -> u64 {
    let mut res = 1;
    let mut x = x % p;
    let mut y = y;
    while y > 0 {
        if y & 1 != 0 {
            res = (res * x) % p;
        }
        y >>= 1;
        x = (x * x) % p;
    }
    res
}

fn miller_rabin(n: u64, a: u64) -> bool {
    let mut r = 0;
    let mut d = n - 1;
    while d % 2 == 0 {
        r += 1;
        d >>= 1;
    }
    let mut x = power(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 0..r - 1 {
        x = power(x, 2, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

fn is_prime(n: u64) -> bool {
    let alist: [u64; 5] = [2, 3, 5, 7, 11];
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    for &a in &alist {
        if n == a {
            return true;
        }
        if !miller_rabin(n, a) {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let n: usize = line.trim().parse().unwrap();
    let mut cnt = 0;
    for _ in 0..n {
        line.clear();
        stdin.read_line(&mut line)?;
        let s: u64 = line.trim().parse().unwrap();
        if is_prime(2 * s + 1) {
            cnt += 1;
        }
    }
    println!("{}", cnt);
    Ok(())
}