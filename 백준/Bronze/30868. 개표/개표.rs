use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let t: i32 = input.trim().parse().unwrap();
    for i in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut n: i32 = input.trim().parse().unwrap();
  
        loop {
            if n <= 0 {
                break;
            }
            if n >= 5 {
                write!(writer, "{} ", "++++").unwrap();
                n -= 5;
            } else {
                write!(writer, "{}", "|").unwrap();
                n -= 1;
            }
        }
        writeln!(writer).unwrap();
    }
}
