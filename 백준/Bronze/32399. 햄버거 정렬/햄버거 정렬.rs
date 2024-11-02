use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let s = line.trim();
        match s {
            ")1(" => {
                writeln!(writer, "{}", 2).unwrap();
            }
            "(1)" => {
                writeln!(writer, "{}", 0).unwrap();
            }
            _=>{
                writeln!(writer, "{}", 1).unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
