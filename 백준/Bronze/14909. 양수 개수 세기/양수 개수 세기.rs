use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let y= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    let mut cnt = 0;
    for i in y{
          if i>0{
            cnt+=1;
          }
    }
    writeln!(writer,"{}",cnt).unwrap();
    writer.flush().unwrap();
}
