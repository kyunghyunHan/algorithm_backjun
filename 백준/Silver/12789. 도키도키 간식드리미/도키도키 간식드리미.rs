use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();
        if let Some(Ok(line)) = input.next() {
            let wait = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut tmp = vec![];
            let mut target = 1;

            for i in wait {
                tmp.push(i);
                while !tmp.is_empty() && *tmp.last().unwrap() == target {
                    tmp.pop();
                    target += 1;
                }
                if tmp.len() > 1 && *tmp.last().unwrap() > tmp[tmp.len() - 2] {
                    writeln!(writer, "Sad").unwrap();
                    return;
                }
            }

            if tmp.is_empty() {
                writeln!(writer, "Nice").unwrap();
            } else {
                writeln!(writer, "Sad").unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
