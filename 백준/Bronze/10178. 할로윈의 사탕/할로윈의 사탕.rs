use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let c= values[0];
        let v= values[1];
        writeln!(writer,"You get {} piece(s) and your dad gets {} piece(s).", c / v, c % v).unwrap();
    }
    writer.flush().unwrap();
}
