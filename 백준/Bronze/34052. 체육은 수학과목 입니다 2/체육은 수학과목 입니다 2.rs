use std::io::{BufRead, BufReader, BufWriter, Write, stdin, stdout};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    let mut v = Vec::new();
    for i in 0..4 {
        reader.read_line(&mut input).unwrap();
        let line = input.trim().parse::<usize>().unwrap();

        v.push(line);
        writer.flush().unwrap();
        input.clear();
    }

    let sum: usize = v.iter().sum();
    if sum + 300 <= 1800 {
        writeln!(writer, "{}", "Yes").unwrap();
    } else {
        writeln!(writer, "{}", "No").unwrap();
    }
}
