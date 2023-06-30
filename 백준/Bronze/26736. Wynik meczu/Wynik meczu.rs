use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::collections::HashMap;
use std::sync::Arc;
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input: String = String::new();
  reader.read_line(&mut input).unwrap();

  let mut x= 0;
  let mut y =0;
  let s= input.trim();
  for i in s.chars(){
    if i=='A'{
   x+=1;
    }else if i =='B'{
      y+=1;
    }
  }
  writeln!(writer,"{} : {}",x,y).unwrap();
}