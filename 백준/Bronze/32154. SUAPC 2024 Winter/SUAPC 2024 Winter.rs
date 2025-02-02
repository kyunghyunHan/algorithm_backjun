use std::io::{self, Write};

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");

    // Initialize the visit array (13 elements)
    let mut visit = vec![false; 13];
    visit[0] = true;
    visit[1] = n == 1 || n == 4 || n == 10;
    visit[2] = true;
    visit[3] = n == 1;
    visit[4] = n != 10;
    visit[5] = true;
    visit[6] = true;
    visit[7] = true;
    visit[8] = n == 2 || n == 3;
    visit[9] = n == 1;
    visit[11] = true;
    visit[12] = true;

    // Count how many problems were solved
    let count = visit.iter().filter(|&&x| x).count();

    // Prepare the result
    let mut result = String::new();
    result.push_str(&format!("{}\nA", count)); // A is always included

    // Add remaining solved problems
    for i in 1..13 {
        if visit[i] {
            result.push_str(&format!(" {}", (b'A' + i as u8) as char));
        }
    }

    // Output the result
    print!("{}", result);
}
