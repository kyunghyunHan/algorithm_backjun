use std::io::{BufReader,BufRead,BufWriter,stdin,stdout,Write};

fn main(){

   let mut reader= BufReader::new(stdin().lock());

   let mut input = String::new();

   reader.read_line(&mut input).unwrap();

   let n :usize= input.trim().parse().unwrap();
let mut writer= BufWriter::new(stdout().lock());
   let mut result= 0;
   for i in 0..n{

      input.clear();
      reader.read_line(&mut input).unwrap();

      let n :usize= input.trim().parse().unwrap();
      result+=n-1;
   }

   writeln!(writer,"{}",result+1).unwrap();
}