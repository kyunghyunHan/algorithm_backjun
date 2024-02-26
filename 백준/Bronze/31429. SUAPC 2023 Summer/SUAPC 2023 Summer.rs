use std::io::{BufRead,BufReader,Write,BufWriter,stdin,stdout};

fn main(){
   let mut reader= BufReader::new(stdin().lock());
   let mut input = String::new();
   let mut writer= BufWriter::new(stdout().lock());
   let pro = vec![12,11,11,10,9,9,9,8,7,6,6];
   let pan  = vec![1600,894,1327,1311,1004,1178,1357,837,1055,556,773];
   reader.read_line(&mut input).unwrap();
   let n= input.trim().parse::<i32>().unwrap();
   writeln!(writer,"{} {}",pro[n as usize-1],pan[n as usize-1]).unwrap();
}