use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.trim().parse::<usize>().unwrap();
        if let Some(Ok(line)) = input.next() {
            let s: Vec<char> = line.trim().chars().collect();

            for i in s {
                if i == 'J' {
                    write!(writer, "{}", 'O').unwrap();
                } else if i == 'O' {
                    write!(writer, "{}", 'I').unwrap();
                } else if i == 'I' {
                    write!(writer, "{}", 'J').unwrap();
                }
            }
        }
    }
    writer.flush().unwrap();
}
