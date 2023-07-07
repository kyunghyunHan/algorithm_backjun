use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input= String::new();
let mut writer= BufWriter::new(stdout().lock());
reader.read_line(&mut input).unwrap();
let mut s:f32= input.trim().parse().unwrap();

input.clear();
reader.read_line(&mut input).unwrap();
let mut a:f32= input.trim().parse().unwrap();
input.clear();
reader.read_line(&mut input).unwrap();
let mut b:f32= input.trim().parse().unwrap();
let price = 250.0;
if s <= a {
    println!("{}", price);
} else {
    let extra = ((s - a) / b).ceil() * 100.0;
    println!("{}", (price + extra) as i32);
}
}
