use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();
        if let Some(Ok(line)) = input.next() {
            let v = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let (mut u, mut d): (usize, usize) = {
                let ud: Vec<usize> = input
                    .next()
                    .unwrap()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                (ud[0], ud[1])
            };
            let mut result = vec![' '; v.len()];
            for (i, &hanger) in v.iter().enumerate() {
                if hanger == 1 && u > 0 {
                    result[i] = 'U';
                    u -= 1;
                } else if hanger == 2 && d > 0 {
                    result[i] = 'D';
                    d -= 1;
                }
            }
            for (i, &hanger) in v.iter().enumerate() {
                if hanger == 3 {
                    if u > 0 {
                        result[i] = 'U';
                        u -= 1;
                    } else if d > 0 {
                        result[i] = 'D';
                        d -= 1;
                    }
                }
            }
            if u == 0 && d == 0 {
                writeln!(writer, "YES").unwrap();
                writeln!(writer, "{}", result.iter().collect::<String>()).unwrap();
            } else {
                writeln!(writer, "NO").unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
