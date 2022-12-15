use std::io;

fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let v: Vec<i32> = input_a
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();

    let n = v[0];

    let mut result = 0;
    for i in 1..n {
        let mut sum = 0;
        let mut num = i;
        while num != 0 {
            sum += num % 10;
            num /= 10;
        }
        if sum + i == n {
            result = i;
            break;
        }
    }
    println!("{}", result);
}
