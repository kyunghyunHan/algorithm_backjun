use std::fmt::Write;
use std::io;

fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let v: Vec<i32> = input_a
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let mut count = 0;
    let mut r = 0;
    let mut test = 0;
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).unwrap();
    let v2: Vec<i32> = input_b
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    for i in 0..v[0] {
        for j in 1..v2[test] + 1 {
            if (v2[test] % j) == 0 {
                count += 1;
            }
        }
        if count == 2 {
            r += 1;
        }
        count = 0;
        test += 1;
    }
    println!("{}", r);
}
