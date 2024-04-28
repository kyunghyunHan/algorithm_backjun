use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn main() {
    let mut reaer = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reaer.read_line(&mut input).unwrap();
    let v = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    let a: i32 = v[0];
    let d: i32 = v[1];
    let k: i32 = v[2];
    let result:String = if (k - a) % d == 0 && (k - a) / d >= 0 {
        ((k - a) / d + 1).to_string()
    } else {
        "X".to_string()
    };

    writeln!(writer,"{}", result).unwrap();
    writer.flush().unwrap();
}
