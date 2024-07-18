use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();
    let n2: i32 = input.trim().parse().unwrap();
    let start_year= n;
    let end_year: i32 = n2;
    for year in (start_year..=end_year).step_by(60) {
        writeln!(writer,"All positions change in year {}", year).unwrap();
    }
}