use std::io::{BufWriter,BufReader,BufRead,Write,stdin,stdout};
fn main(){
   
   let mut index= 0;
   let mut reader= BufReader::new(stdin().lock());
   let mut writer= BufWriter::new(stdout().lock());
   loop{ 
    index+=1;
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    
    if input.trim()=="0"{
          
   break;;
    }else{
  writeln!(writer,"Case {}: Sorting... done!",index).unwrap();
    }

   }



}