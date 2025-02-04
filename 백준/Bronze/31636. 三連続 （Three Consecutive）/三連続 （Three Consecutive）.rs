use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        if let Some(Ok(line)) = input.next() {
            let s = line.trim().to_string();
          if  s.contains("ooo"){
            writeln!(writer,"{}","Yes").unwrap();
          }else{
            writeln!(writer,"{}","No").unwrap();

          }
        }
    }
    writer.flush().unwrap();
}
