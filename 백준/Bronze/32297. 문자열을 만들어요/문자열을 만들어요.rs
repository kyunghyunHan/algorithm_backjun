use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let (Some(Ok(line))) = input.next() {
        let n = line.trim().parse::<usize>().unwrap();
        if let Some(Ok(line)) = input.next() {
            let s = line.trim();
            let result = match s.contains("gori") {
                true => "YES",
                false => "NO",
            };
            writeln!(writer, "{}", result).unwrap();
        } else {
            writeln!(writer, "{}", "ERR").unwrap();
        }
    } else {
        writeln!(writer, "{}", "ERR").unwrap();
    }
    writer.flush().unwrap();
}
