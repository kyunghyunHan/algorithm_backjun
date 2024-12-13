use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let x = line.parse::<u32>().unwrap() % 3;
        match x {
            0 => {
                writeln!(writer, "{}", "S").unwrap();
            }
            1 => {
                writeln!(writer, "{}", "U").unwrap();
            }
            2 => {
                writeln!(writer, "{}", "O").unwrap();
            }
            _ => {
                // 추가된 부분
                writeln!(writer, "{}", "머지").unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
