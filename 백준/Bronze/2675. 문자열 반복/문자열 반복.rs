use std::io;

fn main() {
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).unwrap();
    let v1: Vec<usize> = input_b
        .split_whitespace()
        .map(|x| -> usize { x.parse().unwrap() })
        .collect();

    for i in 0..v1[0] {
        let mut input_a = String::new();
        io::stdin().read_line(&mut input_a).unwrap();
        let s: Vec<&str> = input_a.split_whitespace().collect();
        let num = s[0].parse::<usize>().unwrap();
        let mut std = "".to_string();
        for i in 0..s[1].bytes().len() {
            for j in 0..num {
                std += &s[1][i..i + 1].to_string();
            }
        }
        println!("{}", std);
    }
}