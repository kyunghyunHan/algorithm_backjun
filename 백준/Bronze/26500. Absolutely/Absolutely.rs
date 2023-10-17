
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
   let mut reader= BufReader::new(stdin().lock());
   let mut writer= BufWriter::new(stdout().lock());
   let mut input = String::new();
   reader.read_line(&mut input).unwrap();
   let  n: i64= input.trim().parse().unwrap();
   for _ in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let  nums: Vec<f64>= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect();       
    writeln!(writer,"{:.1}",(nums[0]-nums[1]).abs()).unwrap();
   }
 
   writer.flush().unwrap();
}

