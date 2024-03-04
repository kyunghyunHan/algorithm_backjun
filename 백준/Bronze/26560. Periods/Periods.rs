use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let sm = input.trim();
        let s = sm.chars().collect::<Vec<char>>();
        if s[sm.len() - 1] == '.' {
            writeln!(writer, "{}", sm).unwrap();
        } else {
            writeln!(writer, "{}", sm.to_string()+".").unwrap();
        }
    }
}
