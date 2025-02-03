use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.trim().chars().collect::<Vec<char>>();

        if let Some(Ok(line)) = input.next() {
            let mut count = 0;
            let s = line.trim().chars().collect::<Vec<char>>();
            for i in s {
                match i {
                    'j' | 'i' => {
                        count += 2;
                    }
                    'o' => {
                        count += 1;
                    }
                    _ => {}
                }
            }
            writeln!(writer, "{}", count).unwrap();
        }
    }
    writer.flush().unwrap();
}
