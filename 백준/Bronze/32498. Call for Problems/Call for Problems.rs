use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<u32>().unwrap();
        let mut count = 0;
        for i in 0..n {
            if let Some(Ok(line)) = input.next() {
                let n = line.parse::<u32>().unwrap();
                if n % 2 != 0 {
                    count += 1;
                }
            }
        }
        writeln!(writer, "{}", count).unwrap();

    }

    writer.flush().unwrap();
}
