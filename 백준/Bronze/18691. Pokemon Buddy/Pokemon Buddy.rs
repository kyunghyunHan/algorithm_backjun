use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|line| line.unwrap());

    let case_cnt: usize = input.next().unwrap().parse().unwrap();

    for _ in 0..case_cnt {
        let line = input.next().unwrap();
        let mut values = line.split_whitespace().map(|x| x.parse().unwrap());

        let group: i32 = values.next().unwrap();
        let initial_candies: i32 = values.next().unwrap();
        let evolve_candies: i32 = values.next().unwrap();

        let required_candies = evolve_candies - initial_candies;

        let required_km = match group {
            1 => required_candies,
            2 => 3 * required_candies,
            _ => 5 * required_candies,
        };

        if required_km <= 0 {
            println!("0");
        } else {
            println!("{}", required_km);
        }
    }
}
