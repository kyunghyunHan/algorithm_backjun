use std::io;

fn main() {
    let mut input_d = String::new();
    io::stdin().read_line(&mut input_d).unwrap();
    let v: Vec<i32> = input_d
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut x: Vec<i32> = input
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();

    // x.sort();
    //거꾸로 정렬
    x.sort_by(|a, b| b.cmp(a));
    println!("{:?}", x[v[1] as usize - 1])
}
