use std::io::{BufRead,BufReader,Write,BufWriter,stdin,stdout};

fn main(){
   let mut reader= BufReader::new(stdin().lock());
   let mut input = String::new();
   let mut writer= BufWriter::new(stdout().lock());
   reader.read_line(&mut input).unwrap();
   let n1= input.trim().parse::<i32>().unwrap();
   input.clear();
   reader.read_line(&mut input).unwrap();
   let n2= input.trim().split_whitespace().map(|x|x.to_string()).collect::<Vec<String>>();
   input.clear();
   reader.read_line(&mut input).unwrap();
   let n3= input.trim();
   let mut count =0;
   for i in n2{
      if i == n3{
         count+=1;
      }
   }
   writeln!(writer,"{}",count).unwrap();
   writer.flush().unwrap();
}