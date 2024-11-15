use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let s = line.trim();
        if let Some(Ok(line)) = input.next() {
            let s2 = line.trim();
            let mut cnt = 0;
            let mut pos = 0;
            while let Some(found) = s[pos..].find(s2) {
                cnt += 1;
                pos += found + s2.len();
            }

            writeln!(writer, "{}", cnt).unwrap();
        }
    }
    writer.flush().unwrap();
}
