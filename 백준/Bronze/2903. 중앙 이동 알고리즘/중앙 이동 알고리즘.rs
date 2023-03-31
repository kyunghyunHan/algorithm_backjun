use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
fn main() {
  let stdin = stdin();
  let mut reader = BufReader::new(stdin.lock());
  let mut buffer = String::new();
  reader.read_line(&mut buffer).unwrap();
  let mut n: usize = buffer.trim().parse().unwrap();



  let mut pow = 1;
  for i in 1..=n{
    pow*=2
  }

  





  let stdout = std::io::stdout();
  let mut writer = BufWriter::new(stdout.lock());

  writeln!(writer, "{}", (1+pow)* (1+pow)).unwrap();
  
}