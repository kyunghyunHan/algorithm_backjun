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
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();
            let mut total_pages = 0;
            for p in v {
                if total_pages % 2 == 1 {
                    total_pages += 1;
                }
                total_pages += p;
            }
            writeln!(writer, "{}", (total_pages + 1) / 2).unwrap(); // 올림 적용
        }
    }
    writer.flush().unwrap();
}
