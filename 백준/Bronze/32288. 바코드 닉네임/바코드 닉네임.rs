use std::{
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
    usize,
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();
    match input.next() {
        Some(Ok(line)) => {
            let n = line.trim().parse::<usize>().unwrap();
            match input.next() {
                Some(Ok(line)) => {
                    let s = line
                        .trim()
                        .chars()
                        .map(|x| match x.is_ascii_lowercase() {
                            true => x.to_uppercase().to_string(),
                            false => x.to_lowercase().to_string(),
                        })
                        .collect::<Vec<String>>();

                    
                   for i in s{
                    write!(writer,"{}",i).unwrap();
                   }
                }
                _ => {
                    writeln!(writer, "{}", "ERR").unwrap();
                }
            }
        }
        _ => {
            writeln!(writer, "{}", "ERR").unwrap();
        }
    }
    writer.flush().unwrap();
}
