use std::io;

fn main() {
    let mut inputs = Vec::new();

    for _ in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        inputs.push(input.trim().to_string());
    }

    let mut max_len = 0;
    for input in &inputs {
        if input.len() > max_len {
            max_len = input.len();
        }
    }

    for i in 0..max_len {
        for input in &inputs {
            if i < input.len() {
                print!("{}", input.chars().nth(i).unwrap());
            }
        }
    }
}