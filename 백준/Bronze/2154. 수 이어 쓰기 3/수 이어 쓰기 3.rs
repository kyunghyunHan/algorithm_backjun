use std::{
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
    sync::Arc,
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut write = BufWriter::new(stdout().lock());

    let mut lines = reader.lines();
    let mut s = String::new();

    match lines.next() {
        Some(Ok(line)) => {
            let target = line.trim();
            for i in 1..=1000000 {
                let num = i.to_string();
                s += &num;
            }

            if s.contains(target) {
                let index = s.find(target).unwrap();
                writeln!(write, "{}", index + 1).unwrap();
            }
        }
        _ => {}
    }
    write.flush().unwrap();
}
