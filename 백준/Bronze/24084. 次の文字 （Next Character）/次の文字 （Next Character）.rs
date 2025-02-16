use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();
        if let Some(Ok(line)) = input.next() {
            let s = line.trim().to_string();

            for i in 1..n {
                if s[i..i + 1] == "J".to_string() {
                    writeln!(writer, "{}", &s[i - 1..i]).unwrap();
                }
            }
        }
    }
    writer.flush().unwrap();
}
