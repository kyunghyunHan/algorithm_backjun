use std::io::{stdin, BufRead};

fn main() {
    let input = &mut String::new();
    std::io::stdin().read_line(input).unwrap();
    let numbers: Vec<i32> = input.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    let (e, f, c) = (numbers[0], numbers[1], numbers[2]);
    
    let mut n = (e+f)/c + (e+f)%c;
    let mut res = (e+f)/c;
    
    while n/c > 0 {
    res += n/c;
    n = n/c + n%c;
    }
    
    println!("{}", res);
}
