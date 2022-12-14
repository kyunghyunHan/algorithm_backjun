use std::fmt::Write;
use std::io;
fn find_prime_num(n: i32) -> i32 {
    let mut i = 2;

    if n < 2 {
        return 0;
    } else {
        while i <= (n / i) {
            if n % i == 0 {
                return 0;
            }
            i = i + 1;
        }
        return 1;
    }
}
fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();

    let v: Vec<i32> = input_a
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();

    let mut a = v[0].clone();
    let b = v[1].clone();
    let mut output = String::new();
    while a <= b {
        if find_prime_num(a) == 1 {
            writeln!(output, "{}", a).unwrap();
        }
        a = a + 1;
    }
    print!("{}", output);
}
