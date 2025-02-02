use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    loop {
        if let Some(Ok(line)) = input.next() {
            let v = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<f32>().unwrap())
                .collect::<Vec<f32>>();

            let x = v[0];
            let y = v[1];
            if x == 0. && y == 0. {
                writeln!(writer, "AXIS").unwrap();

                break;
            }
            if x > 0. && y > 0. {
                writeln!(writer, "Q1").unwrap();
            } else if x < 0. && y > 0. {
                writeln!(writer, "Q2").unwrap();
            } else if x < 0. && y < 0. {
                writeln!(writer, "Q3").unwrap();
            } else if x > 0. && y < 0. {
                writeln!(writer, "Q4").unwrap();
            } else {
                writeln!(writer, "AXIS").unwrap();
            }
        }
    }
}
