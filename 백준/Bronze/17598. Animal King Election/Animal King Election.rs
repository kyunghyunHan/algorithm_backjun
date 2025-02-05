use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    let mut l_c = 0;
    let mut t_c = 0;

    for _ in 0..9 {
        if let Some(Ok(line)) = input.next() {
            let n = line.trim();

            if n == "Lion" {
                l_c += 1;
            } else {
                t_c += 1;
            }
        }
    }

    if l_c > t_c {
        writeln!(writer, "Lion").unwrap();
    } else {
        writeln!(writer, "Tiger").unwrap();
    }
    writer.flush().unwrap();
}
