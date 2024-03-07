use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t= input.trim().parse::<i32>().unwrap();
    for i in 0..t{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let n= input.trim().parse::<i32>().unwrap();
        let mut result = 0;
        for k in 1..=n {
            result += k * (k + 1) * (k + 2) / 2;
        }
        writeln!(writer,"{}",result).unwrap();
    }
}
