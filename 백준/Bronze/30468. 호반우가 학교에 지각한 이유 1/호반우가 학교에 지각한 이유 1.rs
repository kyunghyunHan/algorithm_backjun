use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main() {
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input = String::new();

   reader.read_line(&mut input).unwrap();
  let status= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<f32>>();
  let mut str= status[0];
  let mut dex= status[1];
  let mut int= status[2];
  let mut luk= status[3];
  let  n= status[4] as i32;
  let mut count: i32= 0;
   
  while( ((str+dex+int+luk)/4.0) as i32) < n {
     str+=1.0;
    
     count+=1;
  }
  writeln!(writer,"{count}").unwrap();
}  