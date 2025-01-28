use std::cmp;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let mut v = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u64>>();

        v.sort_by(|a, b| b.cmp(a));
        writeln!(writer, "{}", v[1]).unwrap();
    }
    writer.flush().unwrap();
}
