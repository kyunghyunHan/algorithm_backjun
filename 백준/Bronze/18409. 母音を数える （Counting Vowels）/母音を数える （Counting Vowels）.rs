use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let n= input.trim();
  input.clear();
    reader.read_line(&mut input).unwrap();
let s= input.trim();
 let mut count =0;
for i in s.chars(){
 if i =='i'||i=='u'||i=='e'||i=='o'||i=='a'{
    count+=1;
 }
}
writeln!(writer,"{}",count).unwrap();
}