use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};




fn round_number(x: i64) -> i64 {
    let mut power = 10;
    let mut x = x;
    while x >= power {
        x = (x + power / 2) / power * power;
        power *= 10;
    }
    x
}

fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().expect("Failed to parse input");

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let x: i64 = input.trim().parse().expect("Failed to parse input");
        let result = round_number(x);
        writeln!(writer,"{}", result).unwrap();
    }
}