use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    match input.next() {
        Some(Ok(line)) => {
            let case = line.trim().parse::<i32>().unwrap();
            let (mut q1, mut q2, mut q3, mut q4, mut axis) = (0, 0, 0, 0, 0);
            for i in 0..case {
                match input.next() {
                    Some(Ok(lines)) => {
                        let xy = lines
                            .trim()
                            .split_whitespace()
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                        let x = xy[0];
                        let y = xy[1];
                        if x == 0 || y == 0 {
                            axis += 1;
                        } else if x > 0 && y > 0 {
                            q1 += 1;
                        } else if x < 0 && y > 0 {
                            q2 += 1;
                        } else if x < 0 && y < 0 {
                            q3 += 1;
                        } else if x > 0 && y < 0 {
                            q4 += 1;
                        }
                    }
                    _ => writeln!(writer, "{}", "Err").unwrap(),
                }
            }
            writeln!(writer, "Q1: {}", q1).unwrap();
            writeln!(writer, "Q2: {}", q2).unwrap();
            writeln!(writer, "Q3: {}", q3).unwrap();
            writeln!(writer, "Q4: {}", q4).unwrap();
            writeln!(writer, "AXIS: {}", axis).unwrap();


        }
        _ => writeln!(writer, "{}", "Err").unwrap(),
    }
    writer.flush().unwrap();
}
