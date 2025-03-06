use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut count = 0;
    let mut input = reader.lines();
    if let Some(Ok(line)) = input.next() {
        let (n, h) = {
            let nh = line
                .split_whitespace()
                .map(|x| x.parse().expect("error"))
                .collect::<Vec<u64>>();
            (nh[0], nh[1])
        };
        let mut count = 0;

        if let Some(Ok(line)) = input.next() {
            let v = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u64>>();

            for i in v {
                match i {
                    i if i <= h => {
                        count += 1;
                    }
                    _ => {}
                }
            }
        }
        writeln!(writer, "{}", count).unwrap();
    }
}
