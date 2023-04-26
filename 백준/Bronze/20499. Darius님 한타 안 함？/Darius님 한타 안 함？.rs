use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut numbers = input.split('/').map(|x| x.trim().parse().expect("Invalid input"));
    let  k :isize= numbers.next().expect("Invalid input");
    let d = numbers.next().expect("Invalid input");
    let a = numbers.next().expect("Invalid input");
    if k + a < d || d == 0 {
        println!("hasu");
    } else {
        println!("gosu");
    }
}
