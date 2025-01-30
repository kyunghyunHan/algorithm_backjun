use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let mut v = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        v.sort_by(|a, b| a.cmp(b));
        let a = v[0];
        let b = v[1];
        let c = v[2];
        if a == b && b == c {
            writeln!(writer, "{}", 2).unwrap();
        } else if c.pow(2) == a.pow(2) + b.pow(2) {
            writeln!(writer, "{}", 1).unwrap();
        } else {
            writeln!(writer, "{}", 0).unwrap();
        }
    }
}
