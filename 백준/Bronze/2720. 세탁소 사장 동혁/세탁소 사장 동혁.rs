use std::io::{stdin, BufRead};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let t: i32 = buffer.trim().parse().unwrap();

    let quarter = 25;
    let dime = 10;
    let nickel = 5;
    let penny = 1;

    for _ in 0..t {
        let mut answer = vec![0; 4];
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        let mut input: i32 = buffer.trim().parse().unwrap();

        while input > 0 {
            if input >= quarter {
                answer[0] = input / quarter;
                input -= answer[0] * quarter;
            } else if input >= dime {
                answer[1] = input / dime;
                input -= answer[1] * dime;
            } else if input >= nickel {
                answer[2] = input / nickel;
                input -= answer[2] * nickel;
            } else {
                answer[3] = input / penny;
                input -= answer[3] * penny;
            }
        }

        for a in &answer {
            print!("{} ", a);
        }
        println!("");
    }
}