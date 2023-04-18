use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();

    // 입력 데이터의 개수 읽어들이기
    reader.read_line(&mut buffer)?;
    let n: usize = buffer.trim().parse().expect("Failed to parse input");

    let mut names: HashSet<String> = HashSet::new();
    let mut answer = 0;

    for _ in 0..n {
        buffer.clear();
        reader.read_line(&mut buffer)?;
        let nickname = buffer.trim().to_string();
        if nickname == "ENTER" {
            names.clear();
            continue;
        }
        answer += if names.contains(&nickname) { 0 } else { 1 };
        names.insert(nickname);
    }

    println!("{}", answer);

    Ok(())
}