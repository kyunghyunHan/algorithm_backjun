use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){

   let mut reader= BufReader::new(stdin().lock());
   let mut writer= BufWriter::new(stdout().lock());
   let mut input = String::new();
   reader.read_line(&mut input).unwrap();
   let n: usize = input.trim().parse::<usize>().unwrap();
   
   for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s: usize = input.trim().parse::<usize>().unwrap();


        for i in 0..s{
         for j in 0..s{
            if i != 0 && i != s - 1 && j != 0 && j != s - 1{
               write!(writer,"J").unwrap();
            }
            else{
               write!(writer,"#").unwrap();
            }
            

         }
         writeln!(writer).unwrap();
        }
        writeln!(writer).unwrap();

   }
}
