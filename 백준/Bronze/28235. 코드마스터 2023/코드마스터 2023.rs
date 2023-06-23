use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();


    reader.read_line(&mut input).unwrap();

   let s= input.trim();

   if s=="SONGDO"{
    writeln!(writer,"{}","HIGHSCHOOL").unwrap();
   }else if s=="CODE"{
    writeln!(writer,"{}","MASTER").unwrap();

   }else if s=="2023"{
    writeln!(writer,"{}","0611").unwrap();

   }else if s=="ALGORITHM"{
    writeln!(writer,"{}","CONTEST").unwrap();

   }

}