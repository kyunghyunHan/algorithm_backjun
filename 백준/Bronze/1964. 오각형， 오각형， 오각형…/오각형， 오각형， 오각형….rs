use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
   let mut nums= input.trim().split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap());
   let n= nums.next().unwrap();
   let a = (1 + n)*n/2;
   writeln!(writer,"{}",(a*3 + n + 1)%45678).unwrap();

   writer.flush().unwrap();
}