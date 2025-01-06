use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut reader = reader.lines();

    let n = match reader.next() {
        Some(Ok(line)) => line.trim().parse::<usize>().unwrap(),
        _ => {
            panic!("ERR")
        }
    };

    for _ in 0..n {
        let thing = match reader.next() {
            Some(Ok(line)) => line.trim().parse::<f64>().unwrap(),
            _ => {
                panic!("ERR")
            }
        };
        let discounted_price = thing * 0.8;

        writeln!(writer, "${:.2}", discounted_price).unwrap();
    }
    writer.flush().unwrap();
}
