use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        if n % 2 == 0 {
            writeln!(writer, "{}", "SciComLove").unwrap();
        } else {
            writeln!(writer, "{}", "evoLmoCicS").unwrap();
        }
    }
    writer.flush().unwrap();
}
