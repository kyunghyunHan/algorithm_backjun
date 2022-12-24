use std::io;

fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let s: Vec<&str> = input_a.split_whitespace().collect();

    let null = 0;
    println!("{:?}", s.len())
    // for i in s[0].bytes() {}
}
