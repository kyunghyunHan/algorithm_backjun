use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: u32 = iter.next().unwrap().parse().unwrap();
    let b: u32 = iter.next().unwrap().parse().unwrap();

    let mut b_num = String::new();
    let mut tmp = n;
    while tmp != 0 {
        let remainder = tmp % b;
        if remainder > 9 {
            let c = char::from_u32(remainder - 10 + b'A' as u32).unwrap();
            b_num.push(c);
        } else {
            b_num.push(char::from_digit(remainder, 10).unwrap());
        }
        tmp /= b;
    }
    b_num = b_num.chars().rev().collect();

    println!("{}", b_num);
}