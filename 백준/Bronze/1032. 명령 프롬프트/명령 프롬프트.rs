use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let N: usize = line.trim().parse().unwrap();

    line.clear();
    input.read_line(&mut line).unwrap();
    let mut str = line.trim().to_string();

    for _ in 0..(N - 1) {
        line.clear();
        input.read_line(&mut line).unwrap();
        let tmp = line.trim();

        for (j, c) in tmp.chars().enumerate() {
            if c != str.chars().nth(j).unwrap() {
                str.replace_range(j..(j + 1), "?");
            }
        }
    }

    println!("{}", str);
}
