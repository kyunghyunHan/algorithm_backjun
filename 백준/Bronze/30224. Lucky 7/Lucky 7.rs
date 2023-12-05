use std::{io::{BufReader,BufRead,BufWriter,Write,stdin,stdout}, array};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
  let  seven:i32= input.trim().parse().unwrap();
  if seven % 7 == 0 && seven.to_string().contains('7') {
    writeln!(writer,"3").unwrap();
} else if seven % 7 == 0 {
    writeln!(writer,"1").unwrap();
} else if seven.to_string().contains('7') {
    writeln!(writer,"2").unwrap();
} else {
    writeln!(writer,"0").unwrap();
}

  
}