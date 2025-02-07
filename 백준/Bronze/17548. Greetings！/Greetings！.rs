use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let s = line.trim();
        let fiter = s.chars().filter(|x| *x == 'e').count();
        let result = format!("h{}y", "e".repeat(fiter * 2));
        writeln!(writer, "{}", result).unwrap();
    }
    writer.flush().unwrap();
}
