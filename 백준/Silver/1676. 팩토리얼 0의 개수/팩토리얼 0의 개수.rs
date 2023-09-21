use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut v:Vec<i64>= vec![];
    let mut count= 0;
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse::<i64>().unwrap();
    let mul5 = n / 5;
    let mul25 = n / 25;
    let mul125 = n / 125;
    let result = mul5 + mul25 + mul125;

    println!("{}", result);
}