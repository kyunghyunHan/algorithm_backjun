use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
   let n:i32= input.trim().parse().unwrap();
     

     for i in 1..=n{

      if i%6==0 || i==n{
        write!(writer,"{} {} ",i,"Go!").unwrap();

      }else{
        write!(writer,"{} ",i).unwrap();

      }
     }
  
}