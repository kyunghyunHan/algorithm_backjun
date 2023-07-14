use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();
reader.read_line(&mut input).unwrap();
let n = input.trim().parse().unwrap();
println!("{:.8}", f64::sqrt(n) * 4.0);

}
