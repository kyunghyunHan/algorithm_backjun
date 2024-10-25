use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    match input.next() {
        Some(Ok(line)) => {
            let n = line.trim().parse::<u128>().unwrap();

            for _ in 0..n {
                match input.next() {
                    Some(Ok(next_line)) => {
                        let words: Vec<&str> = next_line.trim().split_whitespace().collect();

                        if words.len() == 2 {
                            let first = u128::from_str_radix(words[0], 2).expect("Error1");
                            let second = u128::from_str_radix(words[1], 2).expect("Error2");
                            let result = first + second;

                            writeln!(writer, "{}", format!("{:b}", result)).unwrap();
                        }
                    }
                    _ => {
                        writeln!(writer, "{}", "Error3").unwrap();
                    }
                }
            }
        }
        _ => {
            writeln!(writer, "{}", "Error4").unwrap();
        }
    }
}
