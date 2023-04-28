use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut input_result= input.trim().split_whitespace().map(|x|x.parse::<i128>().unwrap());
    let a= input_result.next().unwrap();
    let b= input_result.next().unwrap();
   let mut count:i128=0;
   let mut writer= BufWriter::new(stdout().lock());

   if a<=b{
writeln!(writer,"{}",(b-a+1)*(b+a)/2).unwrap();
   }else{
    writeln!(writer,"{}",(a-b+1)*(b+a)/2).unwrap();
   }
  


  



}