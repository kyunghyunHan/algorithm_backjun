use std::{io::{stdin, stdout, BufRead, BufReader, BufWriter, Write}, result};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    match input.next() {
        Some(Ok(line)) => {
            let xy = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();
            let x = xy[0];
            let y = xy[1];
        
            match input.next() {
                Some(Ok(line)) => {
                    let n = line.trim().parse::<usize>().unwrap();
                    let mut ant =  x as f32 * (1000./ y as f32);
                    for i in 0..n {
                        match input.next() {
                            Some(Ok(line)) => {
                                let xy = line
                                    .trim()
                                    .split_whitespace()
                                    .map(|x| x.parse().unwrap())
                                    .collect::<Vec<usize>>();
                                let xi = xy[0];
                                let yi = xy[1];
                                let result = xi as f32 * (1000./ yi as f32);
                                if ant>result{
                                    ant = result;
                                }

                            }
                            _ => {
                                writeln!(writer, "ERR").unwrap();
                            }
                        }
                    }
                    writeln!(writer, "{:.2}",ant).unwrap();

                }
                _ => {
                    writeln!(writer, "ERR").unwrap();
                }
            }
        }
        _ => {
            writeln!(writer, "ERR").unwrap();
        }
    }
}
