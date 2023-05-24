use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
   let mut reader= BufReader::new(stdin().lock());

   let mut input=String::new();

   reader.read_line(&mut input).unwrap();
let mut writer= BufWriter::new(stdout().lock());
   let mut n= input.trim().parse().unwrap();
   for i in 0..n{
      input.clear();
      reader.read_line(&mut input).unwrap();
      let mut s= input.trim().split_whitespace();
      let a= s.next().unwrap();
      let b= s.next().unwrap();

       if a!=b{
writeln!(writer,"ERROR").unwrap();
       }  else{
         writeln!(writer,"OK").unwrap();
       }

   }

}