use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut zy = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let (x, y) = (zy[0], zy[1]);

    let z = (100 * y) / x;
    let mut left = 0;
    let mut right = x;
    let mut res = x;
    if z >= 99 {
        writeln!(writer, "{}", -1).unwrap();
        return;
    } else {
        while left <= right {
            let mid = (left + right) / 2;
            if (100 * (y + mid)) / (x + mid) > z {
                res = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        writeln!(writer, "{}", res).unwrap();
    }
    writer.flush().unwrap();
}
