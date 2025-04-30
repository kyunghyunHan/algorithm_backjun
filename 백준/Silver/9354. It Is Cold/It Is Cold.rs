use std::io::{self, Read};

fn solve(n: usize, s: &[i64], c: &[u8]) -> i64 {
    let mut acc_sum = 0;
    let mut started = false;

    for i in (0..n).rev() {
        match (started, c[i]) {
            (false, b'T') => {
                acc_sum += s[i];
                started = true;
            }
            (true, b'T') => acc_sum += s[i],
            (true, b'A') => {
                acc_sum -= s[i];
                if acc_sum < 0 {
                    acc_sum = 0;
                }
            }
            _ => {}
        }
    }

    acc_sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let t: usize = tokens.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = tokens.next().unwrap().parse().unwrap();

        let s: Vec<i64> = (0..n)
            .map(|_| tokens.next().unwrap().parse().unwrap())
            .collect();

        let c_str = tokens.by_ref().take(n).collect::<Vec<_>>();
        let c: Vec<u8> = c_str.iter().map(|s| s.as_bytes()[0]).collect();

        println!("{}", solve(n, &s, &c));
    }
}
