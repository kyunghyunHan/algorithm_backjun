use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let s = line.trim().chars().collect::<Vec<char>>();
        let s2 = ['K', 'O', 'R', 'E', 'A'];
        let mut counter = 0;
        let mut result = 0;
        for i in s {
            if counter == 5 {
                counter = 0;
            }
            if i == s2[counter] {
                result += 1;
                counter += 1;
            }
        }
        writeln!(writer, "{}", result).unwrap();
    }
    writer.flush().unwrap();
}
