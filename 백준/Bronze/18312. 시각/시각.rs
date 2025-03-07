use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let (n, k) = {
            let nk = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (nk[0], nk[1])
        };
        let mut count = 0;
        for h in 0..=n {
            for m in 0..60 {
                for s in 0..60 {
                    let time_str = format!("{:02}{:02}{:02}", h, m, s);
                    if time_str.contains(&k.to_string()) {
                        count += 1;
                    }
                }
            }
        }

        writeln!(writer, "{}", count).unwrap();
    }
    writer.flush().unwrap();
}
