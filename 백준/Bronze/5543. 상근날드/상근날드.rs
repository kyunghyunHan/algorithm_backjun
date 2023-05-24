use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main(){
   let mut reader= BufReader::new(stdin().lock());

  
   let mut ham:Vec<usize>=vec![];
   let mut coke:Vec<usize>=vec![];
   let mut writer= BufWriter::new(stdout().lock());
   for i in 0..5{
      let mut input = String::new();
      reader.read_line(&mut input).unwrap();
      let mut n:usize =input.trim().parse().unwrap();
      if i <3{
         ham.push(n);
      }else{
         coke.push(n)
      }
      
   }

   ham.sort();
   let t= ham[0];
   let s= usize::min(coke[0], coke[1]);

   writeln!(writer,"{}",t+s-50).unwrap();
}