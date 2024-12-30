use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::result::Result;

fn main() -> Result<(), std::io::Error> {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let x = line.parse::<u32>().unwrap() - 2024;
        writeln!(writer, "{}", x)
    } else {
        Ok(())
    }
}
