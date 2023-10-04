use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut days: Vec<i32>= input.trim().split_ascii_whitespace().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    days.sort();
    let sums:i32 = days.iter().sum() ;
     writeln!(writer,"{}",sums-days[days.len()-1]).unwrap();
  writer.flush().unwrap();
}