use std::io::{BufRead,BufReader,Write,BufWriter,stdin,stdout};

fn main(){
   let mut reader= BufReader::new(stdin().lock());
   let mut input = String::new();
   let mut writer= BufWriter::new(stdout().lock());
   reader.read_line(&mut input).unwrap();
   let a= input.trim().parse::<i32>().unwrap();
   input.clear();
   reader.read_line(&mut input).unwrap();
   let b= input.trim().parse::<i32>().unwrap();
   input.clear();
   reader.read_line(&mut input).unwrap();
   let c= input.trim().parse::<i32>().unwrap();
   input.clear();
   reader.read_line(&mut input).unwrap();
   let d= input.trim().parse::<i32>().unwrap();
   input.clear();
   reader.read_line(&mut input).unwrap();
   let e= input.trim().parse::<i32>().unwrap();
   if a<0{
      writeln!(writer,"{}",a.abs()*c+d+b*e).unwrap();
   }else{
      writeln!(writer,"{}",(b-a)*e).unwrap();
   }
   writer.flush().unwrap();
}