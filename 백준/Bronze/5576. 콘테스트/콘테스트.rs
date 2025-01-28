use std::cmp;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout());

    let mut input = reader.lines();
    let mut w = [0usize; 10];
    let mut k = [0usize; 10];
    for i in 0..10 {
        if let Some(Ok(line)) = input.next() {
            let n = line.trim().parse::<usize>().unwrap();
            w[i] = n;
        }
    }
    for i in 0..10 {
        if let Some(Ok(line)) = input.next() {
            let n = line.trim().parse::<usize>().unwrap();
            k[i] = n;
        }
    }
    w.sort_by(|a, b| b.cmp(a));
    k.sort_by(|a, b| b.cmp(a));
    let mut wr = 0;
    let mut kr = 0;
    for i in 0..3 {
        wr += w[i];
        kr += k[i];
    }

    writeln!(writer, "{} {}", wr, kr).unwrap();
    writer.flush().unwrap();
}
