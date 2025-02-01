use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    let mut sums = 1;

    for _ in 0..2 {
        if let Some(Ok(line)) = input.next() {
            let n = line.parse::<u32>().unwrap();

            sums*=n;
        }
    }
    writeln!(writer, "{}", sums).unwrap();
    writer.flush().unwrap();
}
