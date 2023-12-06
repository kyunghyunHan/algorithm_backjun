use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
  let mut n:i32= input.trim().parse().unwrap();
  let s = "WelcomeToSMUPC";
  let result_char = s.chars().nth(((n-1) % 14)as usize).expect("Invalid index");
writeln!(writer,"{}",result_char).unwrap();
writer.flush().unwrap();
}