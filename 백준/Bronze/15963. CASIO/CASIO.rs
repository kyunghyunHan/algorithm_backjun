use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    reader.read_line(&mut input)?;

    let mut nums = input.trim().split_whitespace().map(|x| x.parse::<isize>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    if n == m {
        println!("1");
    } else {
        println!("0");
    }

    Ok(())
}
