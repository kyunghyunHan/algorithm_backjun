use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut lines = reader.lines();

    if let Some(Ok(line)) = lines.next() {
        for i in 0..line.parse::<i32>().unwrap() {
            if let Some(Ok(line)) = lines.next() {
                let line = line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>();
                let v = line[0];
                let e = line[1];

                //면의수 = 2-꼭지점의수 +모서리의수
                let result = 2 - v + e;
                writeln!(writer, "{}", result).unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
