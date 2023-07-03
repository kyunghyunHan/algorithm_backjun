use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut input =String::new();
  let mut writer= BufWriter::new(stdout().lock());
  reader.read_line(&mut input).unwrap();
  let n= input.trim().parse().unwrap();
  /*
  11로 나누어 떨어지면 super
  
   */
  for i in 1..=n{
 

    if i %7==0 && i%11==0{
      writeln!(writer,"{}","Wiwat!").unwrap();

    }else   if i %7 ==0{
writeln!(writer,"{}","Hurra!").unwrap();
    }else if i%11==0 {
      writeln!(writer,"{}","Super!").unwrap();

    }else{
      writeln!(writer,"{}",i).unwrap();

    }
  }
}