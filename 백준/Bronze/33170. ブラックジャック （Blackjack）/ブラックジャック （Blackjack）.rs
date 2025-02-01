use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    let mut sums = 0;

    for _ in 0..35 {
        if let Some(Ok(line)) = input.next() {
            let n = line.parse::<u32>().unwrap();
            sums += n;
        }
    }

    if sums <= 21 {
        writeln!(writer, "{}", 1).unwrap();
    } else {
        writeln!(writer, "{}", 0).unwrap();
    }
    writer.flush().unwrap();
}
