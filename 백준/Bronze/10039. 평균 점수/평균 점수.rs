use std::io::{BufReader,BufRead,BufWriter,stdin,stdout,Write};

fn main(){

   let mut reader= BufReader::new(stdin().lock());


let mut writer= BufWriter::new(stdout().lock());

let mut result= 0;
for i in 0..5{
   
   let mut input = String::new();

   reader.read_line(&mut input).unwrap();

   let n :usize= input.trim().parse().unwrap();

   if n<40{
result+=40  ;
}else{
   result+=n
}
   }
   
writeln!(writer,"{}",result/5).unwrap();
}