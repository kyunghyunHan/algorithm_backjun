use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let ab = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let a = ab[0];
        let b = ab[1];

        if a >= b {
            writeln!(writer, "{}", b).unwrap();
        } else if a < b && b != 0 {
            writeln!(writer, "{}", a + 1).unwrap();
        }
    }
    writer.flush().unwrap();
}
