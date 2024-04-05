use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:usize= input.trim().parse().unwrap();
    writeln!(writer,"{}*", " ".repeat(n - 1))?;
    for i in 0..(n - 1) {
        writeln!(writer,
            "{}{}{}",
            " ".repeat(n - 2 - i),
            "*",
            " ".repeat(2 * i + 1) + "*"
        )?;
    }
    writer.flush()?;
    Ok(())
}
