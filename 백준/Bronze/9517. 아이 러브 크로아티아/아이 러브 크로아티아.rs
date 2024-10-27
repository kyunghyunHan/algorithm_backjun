use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();
    match input.next() {
        Some(Ok(first)) => {
            let mut k = first.trim().parse::<usize>().unwrap();
            match input.next() {
                Some(Ok(second)) => {
                    let mut time = 0;
                    let n = second.trim().parse::<usize>().unwrap();
                    for i in 0..n {
                        match input.next() {
                            Some(Ok(list)) => {
                                let t = list.trim().split_ascii_whitespace().collect::<Vec<&str>>();
                                let t1 = t[0].parse::<usize>().unwrap();
                                let t2 = t[1];
                                time += t1;
                                match time >= 210 {
                                    true => {
                                        writeln!(writer, "{}", k).unwrap();
                                        break;
                                    }
                                    _ => {}
                                }
                                match t2 == "T" {
                                    true => {
                                        k = (k + 1) % 9;
                                        match k==0{
                                            true=>k+=1,
                                            _=>{}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {
                                writeln!(writer, "ERR").unwrap();
                            }
                        }
                    }
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
