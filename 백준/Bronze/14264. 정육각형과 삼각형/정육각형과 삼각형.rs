use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let l:f64= input.trim().parse().unwrap();
    writeln!(writer,"{}",3f64.powf(0.5)/4f64 * l.powf(2.))?;
    writer.flush()?;
    Ok(())
}
