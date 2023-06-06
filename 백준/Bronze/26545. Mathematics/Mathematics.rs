use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){

    let mut reader= BufReader::new(stdin().lock());

    let mut input= String::new();
    reader.read_line(&mut input ).unwrap();
    let n= input.trim().parse().unwrap();
    let mut result= 0;
   for i in 0..n{
    input.clear();
    reader.read_line(&mut input ).unwrap();
    let n:i128= input.trim().parse().unwrap();
   result+=n;
   }
   println!("{}",result);
}
