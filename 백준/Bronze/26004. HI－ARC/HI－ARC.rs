use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read input");

    let mut h = 0;
    let mut i = 0;
    let mut a = 0;
    let mut r = 0;
    let mut c = 0;

    for ch in word.chars() {
        match ch {
            'H' => h += 1,
            'I' => i += 1,
            'A' => a += 1,
            'R' => r += 1,
            'C' => c += 1,
            _ => {}
        }
    }

    println!("{}", vec![h, i, a, r, c].iter().min().unwrap());
}
