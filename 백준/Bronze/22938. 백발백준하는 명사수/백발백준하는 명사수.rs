use std::io;
use std::f64::consts::PI;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let info: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let info2: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let dist = ((info[0] - info2[0]) as f64).hypot((info[1] - info2[1]) as f64);
    let total_radian = (info[2] + info2[2]) as f64;

    if dist >= total_radian {
        println!("NO");
    } else {
        println!("YES");
    }
}
