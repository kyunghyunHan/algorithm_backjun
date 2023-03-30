use std::io::{stdin, BufRead, BufReader, BufWriter, Write, Stdout};

fn main() {
  let stdin= stdin();

  let mut reader= BufReader::new(stdin.lock());

  let mut buffer= String::new();
  reader.read_line(&mut buffer).unwrap();
  let n:usize= buffer.trim().parse().unwrap();
 

 let stdout= std::io::stdout();
 let mut writer= BufWriter::new(stdout.lock());

 writeln!(writer,"{}",1<<n).unwrap();
}