use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();

    let m = n[0];
    let k = n[1];

    if m % k ==0{
        writeln!(writer,"Yes").unwrap();
    }else{
        writeln!(writer,"No").unwrap();

    }
    writer.flush().unwrap();
}   
