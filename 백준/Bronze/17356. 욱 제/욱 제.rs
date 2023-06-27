use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut writet= BufWriter::new(stdout().lock());
    let mut v:Vec<f64>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
     
   
 /*먼저 (B-A)/400 */
   let m=( v[1]-v[0])/400 as f64;
 /* 1/1+10^M */
   let result= 1 as f64/(1 as f64+f64::powf(10 as f64, m));
   writeln!(writet,"{}",result).unwrap();
   
}
