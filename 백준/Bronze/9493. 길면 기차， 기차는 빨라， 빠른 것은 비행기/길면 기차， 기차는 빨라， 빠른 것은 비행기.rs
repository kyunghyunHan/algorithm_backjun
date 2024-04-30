use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    loop {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let v = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<f64>>();
        let (m, a, b) = (v[0], v[1], v[2]);
        if m == 0. && a == 0. && b == 0. {
            break;
        }

        let t = ((m / a - m / b) * 3600.).round() as i32;
        let hours = t / 3600;
        let minutes = (t % 3600) / 60;
        let seconds = t % 60;
        writeln!(writer, "{}:{:02}:{:02}", hours, minutes, seconds).unwrap();
    }

    writer.flush().unwrap();
}
