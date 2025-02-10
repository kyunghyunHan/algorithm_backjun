use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        for _ in 0..n {
            if let Some(Ok(line)) = input.next() {
                let fi = line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let (i, f) = (fi[0], fi[1]);

                if (i <= 1 && f <= 2) || (i <= 2 && f <= 1) {
                    writeln!(writer, "Yes").unwrap();
                }else{
                    writeln!(writer, "No").unwrap();

                }
            }
        }
    }
    writer.flush().unwrap();
}
