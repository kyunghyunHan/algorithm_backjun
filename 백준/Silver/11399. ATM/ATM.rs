use std::io;

fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let n: Vec<i32> = input_a
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).unwrap();
    let mut numbers: Vec<i32> = input_b
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    numbers.sort();
    let mut time = 0;
    let mut tmp = 0;

    for i in 0..n[0] {
        tmp += numbers[i as usize];
        time += tmp;
    }
    println!("{}", time);
}
