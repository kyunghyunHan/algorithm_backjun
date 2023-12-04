use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

  for _ in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m: Vec<i32> = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

   let wi= m[0] * (m[2]-1)+m[1];
   writeln!(writer,"{}",wi).unwrap();




  }
  writer.flush().unwrap();

}