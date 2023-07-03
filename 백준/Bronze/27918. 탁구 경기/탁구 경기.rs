use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());

  let mut input = String::new();
  reader.read_line(&mut input).unwrap();
   
   let mut d =0;
   let mut p =0;

   let n= input.trim().parse().unwrap();
   for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let n= input.trim();

  
       if n== "D"{
        d+=1;
      }else if n =="P"{
        p+=1;
      } 
      if d-p==2 ||p-d==2{
        break;;
      }
    
     

      
    
   }

   writeln!(writer,"{}:{}",d,p).unwrap();
}