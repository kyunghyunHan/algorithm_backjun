use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();

reader.read_line(&mut input).unwrap();
let n :i32= input.trim().parse().unwrap();
input.clear();
reader.read_line(&mut input).unwrap();
let n2:i32= input.trim().parse().unwrap();

let day= n+7*n2;
if day >=1 &&day<=30 {
    writeln!(writer,"{}",1).unwrap();
}else {
    writeln!(writer,"{}",0).unwrap();

}
}
