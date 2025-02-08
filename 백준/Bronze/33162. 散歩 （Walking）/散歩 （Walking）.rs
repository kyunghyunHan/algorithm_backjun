use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();
        let mut cnt = 0;
        //행동 a :3 진행
        //행동 b : 2m 돌아감
        for i in 1..=n {
            if i % 2 != 0 {
                cnt += 3;
            } else {
                cnt -= 2;
            }
        }

        writeln!(writer, "{cnt}").unwrap();
    }
    writer.flush().unwrap();
}
