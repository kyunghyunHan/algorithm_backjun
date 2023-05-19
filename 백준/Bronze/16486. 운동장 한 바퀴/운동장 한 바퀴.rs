use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};


fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input = String::new();
let mut writer= BufWriter::new(stdout().lock());
reader.read_line(&mut input).unwrap();
let d1= input.trim().parse::<f32>().unwrap();

let mut input = String::new();
reader.read_line(&mut input).unwrap();
let d2= input.trim().parse::<f32>().unwrap();

writeln!(writer,"{:.6}",d1*2.0+2.0*3.141592*d2).unwrap();

}