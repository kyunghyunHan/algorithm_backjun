use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
  let mut input = String::new();
reader.read_line(&mut input).unwrap();

let s= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();


let n= s[0];
let w = s[1];
let h= s[2];
let l = s[3];


writeln!(writer,"{}",usize::min((w/l)*(h/l), n)).unwrap();
}