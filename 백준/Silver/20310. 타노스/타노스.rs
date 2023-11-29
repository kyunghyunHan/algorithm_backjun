use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();

    let mut cnt0 = 0;
    let mut cnt1 = 0;

    for ch in input.chars() {
        match ch {
            '0' => cnt0 += 1,
            '1' => cnt1 += 1,
            _ => {}
        }
    }

    cnt0 /= 2;
    cnt1 /= 2;
    let len = input.trim().len() / 2;

    let mut result = String::new();

    for i in 0..len {
        if i < cnt0 {
            result.push('0');
        } else {
            result.push('1');
        }
    }

    writeln!(writer,"{}", result).unwrap();
    writer.flush().unwrap();
}
