use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer = BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let t = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let year = input.trim().parse::<usize>().unwrap();
        let mut num: String = year.clone().to_string();
        num.remove(0);
        num.remove(0);
        if (year + 1) % num.parse::<usize>().unwrap() == 0 {
            writeln!(writer, "{}", "Good").unwrap();
        } else {
            writeln!(writer, "{}", "Bye").unwrap();
        }
    }
    writer.flush().unwrap();
}
