use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    let n = match input.next() {
        Some(Ok(line)) => line.parse::<i32>().unwrap(),
        _ => panic!("ERROR"),
    };
    let mut man1 = 100;
    let mut man2 = 100;
    for _ in 0..n {
        let n = match input.next() {
            Some(Ok(line)) => line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>(),
            _ => panic!("ERROR"),
        };

        if n[0] > n[1] {
            man2 -= n[0];
        } else if n[0] < n[1] {
            man1 -= n[1];
        }
    }
    writeln!(writer, "{}", man1).unwrap();
    writeln!(writer, "{}", man2).unwrap();
    writer.flush().unwrap();
}
