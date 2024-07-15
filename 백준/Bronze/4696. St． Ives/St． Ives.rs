use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    loop{
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let n :f64= input.trim().parse().unwrap();
        if n == 0.0 {
            break;
        }

        let result = 1.0 + n + n.powi(2) + n.powi(3) + n.powi(4);
        writeln!(writer,"{:.2}",result).unwrap();

    }
}
