use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let v = input.chars().collect::<Vec<char>>();
    let mut count = 0;

    for i in v {
        match i {
            'A' | 'a' | 'b' | 'D' | 'd' | 'g' | 'O' | 'o' | 'P' | 'p' | 'Q' | 'q' | 'R' | 'e'
            | '@' => count += 1,
            'B' => count += 2,
            _ => {}
        }
    }
    writeln!(writer, "{}", count).unwrap();
}
