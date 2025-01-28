use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();
        if n == 1 {
            write!(writer, "*").unwrap();
        } else {
            for i in 1..=n {
                for j in 1..=n {
                    if j % 2 == 0 {
                        write!(writer, " ").unwrap();
                    } else {
                        write!(writer, "*").unwrap();
                    }
                }
                writeln!(writer).unwrap();

                for j in 1..=n {
                    if j % 2 == 0 {
                        write!(writer, "*").unwrap();
                    } else {
                        write!(writer, " ").unwrap();
                    }
                }
                writeln!(writer).unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
