use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
  
   let mut v:Vec<f32>= vec![];
   loop{
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim();
    if n =="999"{
        break;
    }
    v.push(n.parse::<f32>().unwrap());
   }    

   for i in 1..v.len(){
      writeln!(writer,"{:.2}",v[i]-v[i-1]).unwrap();
   }
}