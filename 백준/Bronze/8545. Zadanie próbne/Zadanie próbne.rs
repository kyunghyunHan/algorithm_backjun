use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){


let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());

let mut input= String::new();
reader.read_line(&mut input).unwrap();

let s= input.trim().to_string();

for i in s.chars().rev(){

    write!(writer,"{}",i).unwrap();
}


}
