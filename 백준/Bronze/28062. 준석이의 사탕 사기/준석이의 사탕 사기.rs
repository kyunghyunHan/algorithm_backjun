use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read N");
    let n: usize = input.trim().parse().expect("Invalid input for N");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read candy");
    let candy: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    let mut result = 0;
    let mut odd_candy = Vec::new();

    for i in &candy {
        if i % 2 == 1 {
            odd_candy.push(*i);
        } else {
            result += i;
        }
    }

    if odd_candy.len() % 2 == 1 {
        odd_candy.sort_by(|a, b| b.cmp(a));
        odd_candy.pop();
        result += odd_candy.iter().sum::<i32>();
    } else {
        result += odd_candy.iter().sum::<i32>();
    }

    println!("{}", result);
}
