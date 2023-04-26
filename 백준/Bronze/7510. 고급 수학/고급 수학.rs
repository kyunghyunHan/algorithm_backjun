use std::io;
use std::cmp::min;
use std::cmp::max;

fn main() {
    let mut input = String::new();

    // 시나리오 개수 입력 받기
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: u32 = input.trim().parse().expect("Invalid input");

    for i in 0..n {
        let mut sides: Vec<u32> = Vec::new();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        sides = input.trim().split_whitespace().map(|x| x.parse().expect("Invalid input")).collect();

        sides.sort();

        println!("Scenario #{}:", i + 1);

        if sides[0] * sides[0] + sides[1] * sides[1] == sides[2] * sides[2] {
            println!("yes\n");
        } else {
            println!("no\n");
        }
    }
}
