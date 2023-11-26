use std::io;
fn main() {
    let mut s: Vec<usize> = vec![];

    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let v: Vec<usize> = input_a
        .split_whitespace()
        .map(|x| -> usize { x.parse().unwrap() })
        .collect();

    let n = v[0];
    let mut k = v[1];

    for i in 0..n {
        let mut input_b = String::new();
        io::stdin().read_line(&mut input_b).unwrap();
        let v1: Vec<usize> = input_b
            .split_whitespace()
            .map(|x| -> usize { x.parse().unwrap() })
            .collect();
        s.push(v1[0]);
    }

    let mut sum = 0;

    for i in (0..n).rev() {
        sum += k / s[i];
        k = k % s[i];
    }
    println!("{}", sum);
}
