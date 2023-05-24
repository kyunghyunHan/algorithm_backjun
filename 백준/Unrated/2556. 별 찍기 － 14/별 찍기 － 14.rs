use std::io::{BufReader,BufRead,BufWriter,stdin,stdout,Write};

fn main(){

   let mut reader= BufReader::new(stdin().lock());
let mut input = String::new();

let mut writer= BufWriter::new(stdout().lock());

reader.read_line(&mut input ).unwrap();
let n= input.trim().parse().unwrap();
for i in 0..n{
   for i in 0..n{
      print!("*");
   }
   println!()
}
}