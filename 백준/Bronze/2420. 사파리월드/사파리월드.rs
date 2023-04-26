use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut nums = input
        .split_whitespace()
        .map(|x| x.parse::<i64>().expect("Failed to parse input"))
        .collect::<Vec<i64>>();
    
    let n = nums[0];
    let m = nums[1];
    
    let res: i64;
    
    if n > m {
        res = n - m;
    } else {
        res = -n + m;
    }
    
    println!("{}", res);
}
