use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    // 첫 번째 줄: T와 X 입력
    let (t, x) = if let Some(Ok(line)) = input.next() {
        let mut nums = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    } else {
        writeln!(writer, "NO").unwrap();
        return;
    };

    // 두 번째 줄: N 입력
    let n = if let Some(Ok(line)) = input.next() {
        line.parse::<usize>().unwrap()
    } else {
        writeln!(writer, "NO").unwrap();
        return;
    };

    let mut count = 0;

    for _ in 0..n {
        if let Some(Ok(_)) = input.next() {
            if let Some(Ok(line)) = input.next() {
                let v = line
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                if v.contains(&x) {
                    count += 1;
                }
            }
        }
    }

    if count == n {
        writeln!(writer, "YES").unwrap();
    } else {
        writeln!(writer, "NO").unwrap();
    }
    writer.flush().unwrap();
}
