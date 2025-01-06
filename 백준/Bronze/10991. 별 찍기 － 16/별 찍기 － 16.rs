use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        for i in 1..=n {
            let spaces = " ".repeat(n - i);
            let stars = "* ".repeat(i - 1) + "*";
            writeln!(writer, "{}{}", spaces, stars).unwrap();
        }
    }
    writer.flush().unwrap();
}
