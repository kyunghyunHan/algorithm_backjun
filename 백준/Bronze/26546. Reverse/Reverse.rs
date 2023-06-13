use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();

  let n= input.trim().parse().unwrap();
  for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let input = parts[0];
    let i: usize = parts[1].parse().expect("Invalid input");
    let j: usize = parts[2].parse().expect("Invalid input");

     
    let ret = format!("{}{}", &input[..i], &input[j..]);
    println!("{}", ret);
  }
}