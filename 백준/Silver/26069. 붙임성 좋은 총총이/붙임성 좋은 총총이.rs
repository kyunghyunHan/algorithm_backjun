use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut dance: HashMap<String, bool> = HashMap::new();
    dance.insert("ChongChong".to_string(), true);
    let mut answer = 1;

    let mut input = String::new();
    handle.read_line(&mut input)?;
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        handle.read_line(&mut input)?;
        let words: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let a = &words[0];
        let b = &words[1];

        if dance.contains_key(a) {
            if !dance.contains_key(b) {
                dance.insert(b.to_string(), true);
                answer += 1;
            }
        } else if dance.contains_key(b) {
            dance.insert(a.to_string(), true);
            answer += 1;
        }
    }

    println!("{}", answer);

    Ok(())
}
