use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        for i in 0..n {
            if let Some(Ok(line)) = input.next() {
                let v: usize = line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .max()
                    .unwrap();

                writeln!(writer, "Case #{}: {}", i + 1, v).unwrap();
            }
        }
    }
}
