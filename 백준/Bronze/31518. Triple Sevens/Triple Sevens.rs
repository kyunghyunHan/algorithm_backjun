use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let an = line.parse::<usize>().unwrap();
        let mut count = 0;
        for _ in 0..3 {
            if let Some(Ok(line)) = input.next() {
                let nv: Vec<usize> = line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();

                if nv.iter().filter(|x| **x == 7).count() >= 1 {
                    count += 1;
                }
            }
        }

        if count >= 3 {
            writeln!(writer, "777").unwrap();
        } else {
            writeln!(writer, "0").unwrap();
        }
    }
    writer.flush().unwrap();
}
