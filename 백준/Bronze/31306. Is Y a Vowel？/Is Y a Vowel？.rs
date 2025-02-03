use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.trim().chars().collect::<Vec<char>>();
        let mut count = 0;
        let mut y_count = 0;
        for i in n {
            match i {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    count += 1;
                }
                'y' => {
                    y_count += 1;
                }
                _ => {}
            }
        }

        writeln!(writer, "{} {}", count, count + y_count).unwrap();
    }
    writer.flush().unwrap();
}
