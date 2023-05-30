use std::io::{BufReader, BufWriter, BufRead, Write, stdin, stdout};

fn main() {
    let mut reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: u128 = input.trim().parse().unwrap();
    let s = fibo(n);
    writeln!(writer, "{}", s).unwrap();
}

fn fibo(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}
