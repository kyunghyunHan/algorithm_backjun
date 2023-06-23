use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();


    reader.read_line(&mut input).unwrap();


    let s= input.trim().chars().collect::<Vec<char>>();
     
   if s[0]==s[1]{
    writeln!(writer,"{}",1).unwrap();
   }else{
    writeln!(writer,"{}",0).unwrap();

   }
}