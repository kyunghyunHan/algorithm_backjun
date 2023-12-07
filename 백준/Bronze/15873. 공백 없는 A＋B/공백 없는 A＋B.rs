use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main() {
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input = String::new();

   reader.read_line(&mut input).unwrap();
   let s = input.trim();

   if s.chars().nth(1).unwrap() == '0' {
       writeln!(writer,"{}", 10 + s[2..].parse::<i32>().unwrap()).unwrap()
   } else {
       writeln!(writer,"{}", s[0..1].parse::<i32>().unwrap() + s[1..].parse::<i32>().unwrap()).unwrap()
   }
}
