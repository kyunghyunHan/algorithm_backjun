use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
  let n = buffer.trim().parse::<i32>().unwrap();
  buffer.clear();
  reader.read_line(&mut buffer).ok();
  let mut split: Vec<i32> = buffer.split_whitespace()
    .map(|x| x.parse::<i32>().unwrap())
    .collect();
  split.sort();
  let mut sums = split[0];
  let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
  for i in 1..n {
    sums += split[i as usize];
    if (i * (i + 1) / 2) > sums {
        write!(writer, "-1").unwrap();
      return
    }
  }
  if n * (n - 1) / 2 == sums {
    write!(writer, "1").unwrap();
  } else {
    write!(writer, "-1").unwrap();
  }
}