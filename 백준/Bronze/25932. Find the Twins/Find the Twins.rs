use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        for _ in 0..n {
            if let Some(Ok(line)) = input.next() {
                let v = line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let has_mack = v.contains(&18);
                let has_zack = v.contains(&17);

                let result = match (has_mack, has_zack) {
                    (true, true) => "both",
                    (true, false) => "mack",
                    (false, true) => "zack",
                    (false, false) => "none",
                };

                writeln!(writer, "{}", line).unwrap();
                writeln!(writer, "{}", result).unwrap();
                writeln!(writer).unwrap(); // 빈 줄 추가

            }
        }
    }
    writer.flush().unwrap();
}
