use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    if let Some(Ok(line)) = input.next() {
        let n: u32 = line.parse().unwrap();

        let mut found = false;
        for i in 2..=9 {
            for j in 1..=9 {
                if i == n || j == n || i * j == n {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        if found {
            writeln!(writer, "1")?;
        } else {
            writeln!(writer, "0")?;
        }
    }
    Ok(())
}
