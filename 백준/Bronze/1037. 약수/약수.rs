use std::io;

fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let v: Vec<usize> = input_a
        .split_whitespace()
        .map(|x| -> usize { x.parse().unwrap() })
        .collect();
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).unwrap();
    let v2: Vec<usize> = input_b
        .split_whitespace()
        .map(|x| -> usize { x.parse().unwrap() })
        .collect();

    let n = v[0].clone();
    let a: Vec<usize> = Vec::new();
    let mut min = 0;
    let mut max = 0;
    let mut val = 0;

    min = v2[0];
    max = v2[0];

    for i in 0..n {
        if v2[i] > max {
            max = v2[i];
        }
        if v2[i] < min {
            min = v2[i];
        }
    }
    val = min * max;

    println!("{}", val);
}
