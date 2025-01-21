use std::cmp;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.trim().chars().collect::<Vec<char>>();
        let mut first = 'A' as i32;
        let mut time = 0;
        for i in n {
            let mut left = i as i32 - first;
            let mut right = first - i as i32;

            if left < 0 {
                left += 26;
            } else if right < 0 {
                right += 26
            }

            time += left.min(right);
            first = i as i32;
        }

        writeln!(writer, "{}", time).unwrap();
    }
    writer.flush().unwrap();
}
