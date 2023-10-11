use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input =  String::new();
    let mut writer= BufWriter::new(stdout().lock());

    reader.read_line(&mut input).unwrap();
    let mut x:i32= input.trim().parse().unwrap();
    let mut length= 64;
    let mut count= 0;
    while x >0 {
      if length >x {
        length /=2;
      }else {
   count+=1;
 x-=length;
      }
    }
    writeln!(writer,"{}",count).unwrap();
    writer.flush().unwrap();}