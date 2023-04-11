use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();
    
    let mut factorial: u64 = 1;
    for i in 1..=n {
        factorial *= i;
    }
    
    println!("{}", factorial);
}