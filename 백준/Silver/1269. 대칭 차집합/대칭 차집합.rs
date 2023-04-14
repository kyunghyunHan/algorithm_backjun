use std::collections::HashSet;
use std::io::{stdin, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let mut nums = buffer.trim().split_whitespace();
    let a_len = nums.next().unwrap().parse::<usize>().unwrap();
    let b_len = nums.next().unwrap().parse::<usize>().unwrap();

    let mut a_set = HashSet::new();
    let mut buffer_a = String::new();
    reader.read_line(&mut buffer_a).unwrap();
    let mut nums_a = buffer_a.trim().split_whitespace();
    for _ in 0..a_len {
        let a_val = nums_a.next().unwrap().parse::<usize>().unwrap();
        a_set.insert(a_val);
    }

    let mut b_set = HashSet::new();
    let mut buffer_b = String::new();
    reader.read_line(&mut buffer_b).unwrap();
    let mut nums_b = buffer_b.trim().split_whitespace();
    for _ in 0..b_len {
        let b_val = nums_b.next().unwrap().parse::<usize>().unwrap();
        b_set.insert(b_val);
    }

    let a_diff = a_set.difference(&b_set).count();
    let b_diff = b_set.difference(&a_set).count();
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}", a_diff + b_diff).unwrap();
}