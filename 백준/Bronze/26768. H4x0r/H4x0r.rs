use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
  let mut writer= BufWriter::new(stdout().lock());

  let mut s = input.trim().to_string();
  let replacements = [("a", "4"), ("e", "3"),("i", "1"),("o", "0"),("s", "5")];

  for (old, new) in replacements.iter() {
      s = s.replace(old, new);
  }

  writeln!(writer,"{}",s).unwrap();
}   