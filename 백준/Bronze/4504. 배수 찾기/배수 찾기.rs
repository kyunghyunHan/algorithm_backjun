use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};

fn main(){
   let mut reader= BufReader::new(stdin().lock());
   let mut input = String::new();
   reader.read_line(&mut input).unwrap();
   let mut n= input.trim().parse::<usize>().unwrap();
   let mut writer= BufWriter::new(stdout().lock());
   loop{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut s= input.trim().parse::<usize>().unwrap();
    if s==0{
        break;
    }
    if s % n==0{
         writeln!(writer,"{} is a multiple of {}.",s,n).unwrap();
    }else{
        writeln!(writer,"{} is NOT a multiple of {}.",s,n).unwrap();
    }

   }
    


}