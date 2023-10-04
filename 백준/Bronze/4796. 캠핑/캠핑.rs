use std::io;

fn main() {
    let mut i = 0;
    loop {
        i += 1;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        
        let tokens: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse integer"))
            .collect();

        let l = tokens[0];
        let p = tokens[1];
        let v = tokens[2];
        
        if l == 0 && p == 0 && v == 0 {
            break;
        }
        
        let a = v / p;
        let b = v % p;
        let result = if l < b { l } else { b } + a * l;
        
        println!("Case {}: {}", i, result);
    }
}