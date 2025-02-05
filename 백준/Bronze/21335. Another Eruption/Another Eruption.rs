use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<f64>().unwrap();
        let pi = std::f64::consts::PI;
        let result = ((n / pi).sqrt()) * 2.0 * pi;

        writeln!(writer, "{}", result).unwrap();
    }
    writer.flush().unwrap();
}
