use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();
    if let Some(Ok(list)) = input.next() {
        let mut v = list.trim().split_whitespace();
        let n = v.next().unwrap().parse::<usize>().unwrap();
        let a = v.next().unwrap().parse::<usize>().unwrap();
        let b = v.next().unwrap().parse::<usize>().unwrap();
        let mut result = [1, 1];

        for i in 0..n {
            result[0] += a;
            result[1] += b;
            if result[0] < result[1] {
                result.swap(0, 1);
            } else if result[0] == result[1] {
                result[1] -= 1;
            }
        }

        writeln!(writer, "{} {}", result[0], result[1]).unwrap();
    } else {
        writeln!(writer, "{}", "ERR!").unwrap();
    }
    writer.flush().unwrap();
}
