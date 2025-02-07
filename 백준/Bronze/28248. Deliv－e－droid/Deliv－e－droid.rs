use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut wrier = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let p = line.parse::<i32>().unwrap();
        if let Some(Ok(line)) = input.next() {
            let c = line.parse::<i32>().unwrap();

            let mut result = p * 50 - c * 10;
            if p > c {
                result += 500;
            }
            writeln!(wrier, "{}", result).unwrap();
        }
    }
    wrier.flush().unwrap();
}
