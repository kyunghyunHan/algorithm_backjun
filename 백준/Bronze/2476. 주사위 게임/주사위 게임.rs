use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut input = String::new();
    reader
        .read_line(&mut input)
        .expect("Failed to read line from input");
    let case: i32 = input.trim().parse().unwrap();

    let mut answer = 0;
    for _ in 0..case {
        input.clear();
        reader
            .read_line(&mut input)
            .expect("Failed to read line from input");
        let mut iter = input.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        let c: i32 = iter.next().unwrap().parse().unwrap();

        if a == b && b == c {
            answer = answer.max(10000 + a * 1000);
        } else if a == b || a == c || b == c {
            answer = answer.max(1000 + a.max(b).max(c) * 100);
        } else {
            answer = answer.max(a.max(b).max(c) * 100);
        }
    }

    writeln!(writer, "{}", answer).unwrap();
    writer.flush().unwrap();
}
