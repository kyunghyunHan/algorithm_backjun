use std::io;

fn friends() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let nums: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse input"))
            .collect();
        let n = nums[0];
        let m = nums[1];

        if n == 0 && m == 0 {
            break;
        }

        println!("{}", n + m);
    }
}

fn main() {
    friends();
}
