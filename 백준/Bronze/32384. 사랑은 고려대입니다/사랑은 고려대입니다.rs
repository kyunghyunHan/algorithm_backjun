use std::error::Error;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();
    match input.next() {
        Some(Ok(line)) => {
            let n = line.trim().parse::<usize>()?;
            for i in 0..n {
                write!(writer,"{} ","LoveisKoreaUniversity" )?;
            }
        }
        _ => {
            writeln!(writer, "{}", "Error: Input not defined")?;
        }
    }
    Ok(())
}
