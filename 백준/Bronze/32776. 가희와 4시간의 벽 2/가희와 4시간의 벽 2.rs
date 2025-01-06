use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
 
    let s_ab: i32 = input.trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();
    let times: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let m_a = times[0];
    let f_ab = times[1];
    let m_b = times[2];

    let flight_time = m_a + f_ab + m_b;
    if s_ab <= flight_time || s_ab <= 240 {
        writeln!(writer, "high speed rail").unwrap();
    } else {
        writeln!(writer, "flight").unwrap();
    }
    writer.flush().unwrap();
}
