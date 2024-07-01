use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let k = input.trim();

    let mut odd = 0;
    let mut even = 0;

    for c in k.chars() {
        let num = c.to_digit(10).expect("Failed to convert char to digit");
        if num % 2 != 0 {
            odd += 1;
        } else {
            even += 1;
        }
    }
    if odd > even {
        writeln!(writer, "1").unwrap();
    } else if odd < even {
        writeln!(writer, "0").unwrap();
    } else {
        writeln!(writer, "-1").unwrap();
    }
    writer.flush().unwrap();
}
