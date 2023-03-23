use std::collections::HashMap;
use std::io;

fn main() {
    let mut mp: HashMap<&str, f64> = HashMap::new();
    mp.insert("A+", 4.5);
    mp.insert("A0", 4.0);
    mp.insert("B+", 3.5);
    mp.insert("B0", 3.0);
    mp.insert("C+", 2.5);
    mp.insert("C0", 2.0);
    mp.insert("D+", 1.5);
    mp.insert("D0", 1.0);
    mp.insert("F", 0.0);

    let mut result: f64 = 0.0;
    let mut score_sum: f64 = 0.0;

    for _ in 0..20 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let tokens: Vec<&str> = input.split_whitespace().collect();
        let name = tokens[0];
        let score: f64 = tokens[1].parse().expect("Failed to parse score");
        let grade = tokens[2];
        if grade == "P" {
            continue;
        }
        result += score * mp.get(grade).unwrap();
        score_sum += score;
    }
    result /= score_sum;

    println!("{:.4}", result);
}