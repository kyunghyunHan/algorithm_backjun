use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut numbers = input.split_whitespace().map(|x| x.trim().parse().expect("Invalid input"));
    let s:isize = numbers.next().expect("Invalid input");
    let k = numbers.next().expect("Invalid input");
    let h = numbers.next().expect("Invalid input");
    
    if s + k + h >= 100 {
        println!("OK");
    } else {
        if s < k && s < h {
            println!("Soongsil");
        } else if k < s && k < h {
            println!("Korea");
        } else if h < s && h < k {
            println!("Hanyang");
        }
    }
}
