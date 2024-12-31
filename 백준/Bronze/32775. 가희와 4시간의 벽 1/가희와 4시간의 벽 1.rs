use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    if let Some(Ok(line)) = input.next() {
        let s: u32 = line.parse().unwrap();
        if let Some(Ok(line)) = input.next() {
            let f: u32 = line.parse().unwrap();

            if s > f {
                writeln!(writer, "{}", "flight").unwrap();
            } else {
                writeln!(writer, "{}", "high speed rail").unwrap();
            }
        }
    }
    writer.flush().unwrap();
    Ok(())
}
