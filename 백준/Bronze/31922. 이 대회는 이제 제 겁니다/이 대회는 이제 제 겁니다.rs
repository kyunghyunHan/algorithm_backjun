use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let nums = input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    if nums[0] +nums[2]> nums[1] {
        writeln!(writer, "{}", nums[0] + nums[2]).unwrap();
    } else {
        writeln!(writer, "{}", nums[1] ).unwrap();
    }
    writer.flush().unwrap();
}
