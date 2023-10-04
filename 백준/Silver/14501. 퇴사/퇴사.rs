use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let i: usize = input.trim().parse().expect("Invalid input");

    let mut l = vec![0; 18];
    let mut c = vec![0; 18];
    let mut r = vec![0; 18];

    for j in 1..=i {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let parts: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        l[j] = parts[0];
        c[j] = parts[1];
    }

    for j in (1..=i).rev() {
        if l[j] > i - j + 1 {
            r[j] = r[j + 1];
        } else {
            r[j] = std::cmp::max(c[j] + r[j + l[j]], r[j + 1]);
        }
    }

    println!("{}", r[1]);
}
