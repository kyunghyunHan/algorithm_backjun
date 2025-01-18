use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();
        if let Some(Ok(line)) = input.next() {
            let v = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if let Some(Ok(line)) = input.next() {
                let v2 = line
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                let mut sum = 0;
                for i in 0..n {
                    sum += (v[i] - v2[i]).abs();
                }

                writeln!(writer,"{}",sum/2).unwrap();
            }
        }
    }

    writer.flush().unwrap();
}
