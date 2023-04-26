use std::io::{BufWriter, BufReader, BufRead, Write, stdin, stdout};
use std::collections::HashMap;

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
 
   
  let mut count = 0;
   for  i in 1..=3{
    let mut input = String::new();
      reader.read_line(&mut input).unwrap();
      let mut num = input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
      let s: usize= num.next().unwrap();

     if i==1{
        count+=s*1;
     }else if i==2{
        count+=s*2;
     }else if i==3{
        count+=s*3;
     }

   }
    
   

    if count >=10{
         writeln!(writer,"{}","happy").unwrap();
    }else{
        writeln!(writer,"{}","sad").unwrap();

    }

    

    
}
