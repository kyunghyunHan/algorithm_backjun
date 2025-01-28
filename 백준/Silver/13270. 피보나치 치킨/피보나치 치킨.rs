use std::{
    cmp::{max, min},
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        if n % 2 != 0 {
            write!(writer,"{} ", 2 + (n - 3) / 2).unwrap();
        } else {
            write!(writer,"{} ", n / 2).unwrap();
        }
    
        if n % 3 == 1 {
            write!(writer,"{}", 2 + (n - 4) / 3 * 2).unwrap();
        } else if n % 3 == 2 {
            write!(writer,"{}", 1 + (n - 2) / 3 * 2).unwrap();
        } else {
            write!(writer,"{}", n / 3 * 2).unwrap();
        }
    }
    writer.flush().unwrap();
}
