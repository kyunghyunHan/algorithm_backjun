use std::collections::VecDeque;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut iter = input_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut visited = vec![false; 101];
    let mut jump = HashMap::new();

    for _ in 0..(n + m) {
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).unwrap();
        let mut iter = tmp.split_whitespace();
        let key: usize = iter.next().unwrap().parse().unwrap();
        let value: usize = iter.next().unwrap().parse().unwrap();
        jump.insert(key, value);
    }

    let mut queue = VecDeque::new();
    queue.push_back((1, 0)); // current position, dice rolls

    visited[1] = true;

    while let Some(v) = queue.pop_front() {
        if v.0 == 100 {
            println!("{}", v.1);
            return;
        }

        for i in 1..=6 {
            let nv = v.0 + i;

            if nv > 100 {
                continue;
            }

            if visited[nv] {
                continue;
            }

            if jump.contains_key(&nv) {
                let mut next_position = jump[&nv];
                if !visited[next_position] {
                    visited[next_position] = true;
                    queue.push_back((next_position, v.1 + 1));
                }
            } else {
                visited[nv] = true;
                queue.push_back((nv, v.1 + 1));
            }
        }
    }
}
