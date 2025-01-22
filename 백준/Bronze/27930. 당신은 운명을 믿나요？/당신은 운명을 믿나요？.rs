use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let mut korea = ['K', 'O', 'R', 'E', 'A'];
        let mut YONSEI = ['Y', 'O', 'N', 'S', 'E', 'I'];
        let s = line.trim().chars().collect::<Vec<char>>();

        let mut yi = 0;
        let mut ki = 0;
        let mut i = 0;
        loop {
            if ki == 4 {
                writeln!(writer, "KOREA").unwrap();
                break;
            }
            if yi == 5 {
                writeln!(writer, "YONSEI").unwrap();
                break;
            }
            if s[i] == korea[ki] {
                ki += 1;
            }
            if s[i] == YONSEI[yi] {
                yi += 1;
            }
            i += 1;
        }
    }
}
