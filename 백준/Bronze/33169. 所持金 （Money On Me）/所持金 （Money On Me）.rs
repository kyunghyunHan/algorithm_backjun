use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    let mut sums = 0;

    for i in 0..2 {
        if let Some(Ok(line)) = input.next() {
            let n = line.parse::<u32>().unwrap();

            if i == 0 {
                sums += (n * 1000);
            } else {
                sums += (n * 10000);

            }
        }
    }
    writeln!(writer, "{}", sums).unwrap();
    writer.flush().unwrap();
}
