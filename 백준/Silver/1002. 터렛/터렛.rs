use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut num = String::new();
    handle.read_line(&mut num)?;
    let num: usize = num.trim().parse().expect("Failed to parse input");

    for _ in 0..num {
        let mut input = String::new();
        handle.read_line(&mut input)?;
        let input: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse input"))
            .collect();
        let x1 = input[0];
        let y1 = input[1];
        let r1 = input[2];
        let x2 = input[3];
        let y2 = input[4];
        let r2 = input[5];

        let d = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
        let cond1 = (r1 - r2) * (r1 - r2);
        let cond2 = (r1 + r2) * (r1 + r2);

        if d == 0 {
            if cond1 == 0 {
                println!("-1");
            } else {
                println!("0");
            }
        } else if d == cond1 || d == cond2 {
            println!("1");
        } else if cond1 < d && d < cond2 {
            println!("2");
        } else {
            println!("0");
        }
    }

    Ok(())
}
