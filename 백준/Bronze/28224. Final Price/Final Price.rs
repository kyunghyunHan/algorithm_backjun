use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());

let mut input = String::new();
reader.read_line(&mut input).unwrap();
let n:i32= input.trim().parse().unwrap();
let mut result :i32= 0;
for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let s:i32= input.trim().parse().unwrap();
    result+=s;
}
writeln!(writer,"{}",result).unwrap();
}
