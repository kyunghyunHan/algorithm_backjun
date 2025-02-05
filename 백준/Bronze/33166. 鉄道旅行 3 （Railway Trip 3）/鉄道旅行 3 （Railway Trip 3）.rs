use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let mut v = line.split_whitespace();
        let p = v.next().unwrap().parse::<i32>().unwrap();
        let q = v.next().unwrap().parse::<i32>().unwrap();

        if let Some(Ok(line)) = input.next() {
            let mut v = line.split_whitespace();
            let a = v.next().unwrap().parse::<i32>().unwrap();
            let b = v.next().unwrap().parse::<i32>().unwrap();
            if q-p <= 0{
              writeln!(writer, "{}", q * a).unwrap();

            }else{
              writeln!(writer, "{}", p * a + (q - p) * b).unwrap();

            }
        }
    }
    writer.flush().unwrap();
}
