use std::collections::HashSet;
use std::io::{BufWriter, BufReader, BufRead, Write, stdin, stdout};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut set: HashSet<u32> = HashSet::new();
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().unwrap();

    for _ in 0..m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let temp: Vec<&str> = input.trim().split_whitespace().collect();

        if temp.len() == 1 {
            if temp[0] == "all" {
                set = (1..=20).collect();
            } else {
                set.clear();
            }
        } else {
            let func = temp[0];
            let x: u32 = temp[1].parse().unwrap();

            match func {
                "add" => {
                    set.insert(x);
                }
                "remove" => {
                    set.remove(&x);
                }
                "check" => {
                    let result = set.contains(&x);
                    writeln!(writer, "{}", if result { 1 } else { 0 }).unwrap();
                }
                "toggle" => {
                    if set.contains(&x) {
                        set.remove(&x);
                    } else {
                        set.insert(x);
                    }
                }
                _ => {}
            }
        }
    }
    writer.flush().unwrap();
}
