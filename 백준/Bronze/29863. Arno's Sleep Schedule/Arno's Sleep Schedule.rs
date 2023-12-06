use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let s: i32 = input.trim().parse().unwrap();
    let mut sleep_length = s- n;
    if sleep_length < 0 {
        sleep_length += 24;
    }
    writeln!(writer,"{}", sleep_length).unwrap();

}