use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Error, Result, Write};

fn main() -> Result<()> {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut reader = reader.lines();

    if let Some(Ok(line)) = reader.next() {
        let team1 = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        let result = team1[0] * 1 + team1[1] * 2 + team1[2] * 3;

        if let Some(Ok(line)) = reader.next() {
            let team2 = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            let result2 = team2[0] * 1 + team2[1] * 2 + team2[2] * 3;

            if result > result2 {
                writeln!(writer, "1")?;
            } else if result < result2 {
                writeln!(writer, "2")?;
            } else {
                writeln!(writer, "0")?;
            }
        }
    }
    writer.flush()?;
    Ok(())
}
