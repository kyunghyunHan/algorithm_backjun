use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut n: i32 = input.trim().parse::<i32>().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let  s= input.trim();
    let mut count = 0.0 ;
     let  mut  result= true;  
  for i in s.chars(){
      if i =='L'{
        count+=0.5;
        result=false;
      }else if i =='S' {
        count+=1.0;

      }
  }
  match result  {
    true=>writeln!(writer,"{}",count  as usize).unwrap(),
    false=>writeln!(writer,"{}",(count+1.0) as usize).unwrap()
      
  }
  writer.flush().unwrap();
}