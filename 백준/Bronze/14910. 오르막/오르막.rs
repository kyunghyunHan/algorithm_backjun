use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let inp: Vec<i32> = input.split_whitespace()
                             .map(|x| x.parse().unwrap())
                             .collect();
    if inp.len() == 1 {
        println!("Good");
    } else {
        let mut cnt = inp[0];
        let mut result = 0;
        for i in 1..inp.len() {
            if inp[i] < cnt {
                println!("Bad");
                result = 1;
                break;
            }
            cnt = inp[i];
        }
        if result == 0 {
            println!("Good");
        }
    }
}
