use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){

    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
   let n :u128= input.trim().parse().unwrap();
   let mut result =1;
   input.clear();
   reader.read_line(&mut input).unwrap();
   let n2 :u128= input.trim().parse().unwrap();
   
   if n>n2{
    writeln!(writer,"{}",1).unwrap();

   }else if n==n2{
    writeln!(writer,"{}",0).unwrap();

   }else{
    writeln!(writer,"{}",-1).unwrap();

   }

}
