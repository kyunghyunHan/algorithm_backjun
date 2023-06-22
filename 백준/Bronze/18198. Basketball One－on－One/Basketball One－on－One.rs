use std::io;

fn main() {
    let mut record = String::new();
    io::stdin().read_line(&mut record).expect("Failed to read input");
    record = record.trim().to_string();

    let mut alice_score = 0;
    let mut barbara_score = 0;

    let mut winner = None;

    for i in (0..record.len()).step_by(2) {
        let player: char = record.chars().nth(i).unwrap();
        let points: i32 = record.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i32;

        if player == 'A' {
            alice_score += points;
        } else {
            barbara_score += points;
        }

        if alice_score >= 11 && alice_score - barbara_score >= 2 {
            winner = Some('A');
            break;
        } else if barbara_score >= 11 && barbara_score - alice_score >= 2 {
            winner = Some('B');
            break;
        }
    }

    if let Some(w) = winner {
        println!("{}", w);
    }
}
