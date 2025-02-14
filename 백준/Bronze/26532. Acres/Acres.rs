use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    if let Some(Ok(line)) = input.next() {
        let (x, y) = {
            let xy = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (xy[0], xy[1])
        };
        let mut result = x * y;
        let mut count = 1;
        let acres = 4840 * 5;

        loop {
            if result > acres {
                result -= acres;
                count += 1;
            } else {
                break;
            }
        }

        writeln!(writer, "{}", count).unwrap();
    }
    writer.flush().unwrap();
}
