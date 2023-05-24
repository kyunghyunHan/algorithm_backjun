use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){

   let mut reader= BufReader::new(stdin().lock());


  loop{
    let mut input= String::new();
let mut writer= BufWriter::new(stdout().lock());
   reader.read_line(&mut input).unwrap();
   let n:f32= input.trim().parse().unwrap();

 if n ==-1.0{
   break;
 }else{
   writeln!(writer,"Objects weighing {:.2} on Earth will weigh {:.2} on the moon.",n,n*0.167).unwrap();

 }
  }

  
   
}