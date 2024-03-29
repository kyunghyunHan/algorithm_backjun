use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let gun: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let xor_result = gun[0] ^ gun[1];
    let res = u32::from_str_radix(&format!("{:b}", xor_result), 2).unwrap();
    writeln!(writer,"{}", res).unwrap();
    writer.flush().unwrap();
}
