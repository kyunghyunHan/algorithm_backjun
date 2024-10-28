use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    match input.next() {
        Some(Ok(line)) => {
            let mut lines = line.split_whitespace();
            let n: usize = lines.next().unwrap().parse().unwrap();
            let m: usize = lines.next().unwrap().parse().unwrap();
            let k: usize = lines.next().unwrap().parse().unwrap();

            writeln!(writer,"{}",(m*k)+m).unwrap();
        }
        _ => {
            writeln!(writer, "{}", "ERR").unwrap();
        }
    }
    writer.flush().unwrap();
}
