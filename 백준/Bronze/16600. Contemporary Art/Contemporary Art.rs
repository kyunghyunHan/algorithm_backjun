use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<f64>().unwrap();

        writeln!(writer, "{:.8}", n.powf(0.5) * 4.).unwrap();
    }
    writer.flush().unwrap();
}
