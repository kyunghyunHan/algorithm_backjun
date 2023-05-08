use std::io::{self, BufRead};

fn go(s: &mut [u8], start: usize, end: usize) {
    if end - start == 1 {
        return;
    }

    let k = (end - start) / 3;
    for i in start + k..end - k {
        s[i] = b' ';
    }

    go(s, start, start + k);
    go(s, end - k, end);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    while let Some(line) = lines.next() {
        let n: usize = line.trim().parse().unwrap();
        let size = 3usize.pow(n as u32);

        let mut s = vec![b'-'; size];
        go(&mut s, 0, size);

        let output = String::from_utf8(s).unwrap();
        println!("{}", output);
    }
}
