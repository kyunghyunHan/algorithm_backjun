use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let mut x = 1;
        let mut y = 1;
        let n = line.parse::<usize>().unwrap();
        for i in 3..=n {
            let z = y;
            y = (x + y) % 1_000_000_007;
            x = z;
        }
        writeln!(writer, "{} {}", y, n - 2).unwrap();
    }
    writer.flush().unwrap();
}
