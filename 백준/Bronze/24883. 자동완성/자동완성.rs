use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut input= String::new();
  let mut writer= BufWriter::new(stdout().lock());
  reader.read_line(&mut input).unwrap();

  let s= input.trim();
  /*

   알파벳이 N또는 ndlaus Naver D2
  아니면 Naver Whale
   */

  if s=="n"||s=="N"{
    writeln!(writer,"{}","Naver D2").unwrap();
  }else{
    writeln!(writer,"{}","Naver Whale").unwrap();
  }
}
