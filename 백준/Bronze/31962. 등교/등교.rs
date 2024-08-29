use std::{
    cmp::min,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let nx = &input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let n = nx[0];
    let x = nx[1];
    let mut result = (0, 0);
    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let st = &input
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        let s = st[0];
        let t = st[1];
        if result.0 < s && s + t <= x {
            result.0 = s;
            result.1 = t;
        }
    }
    if result != (0, 0) {
        writeln!(writer, "{:?}", result.0 ).unwrap();
    } else {
        writeln!(writer, "{:?}", -1).unwrap();
    }
    writer.flush().unwrap();
}
